package particle

import (
	"strings"
)

type World []Particle

func Parse(input string) World {
	var particles []Particle

	for i, line := range strings.Split(input, "\n") {
		particles = append(particles, ParseParticle(i, line))
	}

	return particles
}

func (w *World) Tick() {
	for i := range *w {
		(*w)[i].Tick()
	}
}

func (w *World) RemoveCollisions() {
	particles := *w

	crashes := make(map[Vec3]bool)

	for i, p1 := range particles {
		for _, p2 := range particles[i+1:] {
			if p1.P == p2.P {
				crashes[p1.P] = true
				break
			}
		}
	}

	i := 0
	for i < len(particles) {
		if crashes[particles[i].P] {
			if i < len(particles)-1 {
				copy(particles[i:], particles[i+1:])
			}
			particles = particles[:len(particles)-1]
			continue
		}
		i++
	}

	*w = particles
}

func (w *World) FindClosestOrigin() *Particle {
	particles := *w

	closest := &particles[0]

	for i := range particles[1:] {
		if particles[i].DistanceOrigin() < closest.DistanceOrigin() {
			closest = &particles[i]
		}
	}

	return closest
}
