package particle

import (
	"strings"
)

type World []Particle

func ParseWorld(input string) World {
	var particles []Particle

	for i, line := range strings.Split(input, "\n") {
		particles = append(particles, parseParticle(i, line))
	}

	return particles
}

func (w *World) Tick() {
	for i := range *w {
		(*w)[i].tick()
	}
}

func (w *World) RemoveCollisions() {
	crashSites := make(map[Vec3]bool)

	for i, p1 := range *w {
		for _, p2 := range (*w)[i+1:] {
			if p1.pos == p2.pos {
				crashSites[p1.pos] = true
				break
			}
		}
	}

	for i := 0; i < len(*w); {
		if crashSites[(*w)[i].pos] {
			if i < len((*w))-1 {
				copy((*w)[i:], (*w)[i+1:])
			}
			(*w) = (*w)[:len((*w))-1]
			continue
		}
		i++
	}

	*w = (*w)
}

func (w *World) FindClosestToOrigin() (closest *Particle) {
	for i := range (*w)[1:] {
		if closest == nil || (*w)[i].distanceFromOrigin() < closest.distanceFromOrigin() {
			closest = &(*w)[i]
		}
	}
	return
}
