package particle

import (
	"fmt"
	"strings"
)

type Particle struct {
	ID            int
	acc, vel, pos Vec3
}

func parseParticle(id int, input string) (p Particle) {
	p.ID = id

	input = strings.ReplaceAll(input, " ", "")

	_, err := fmt.Sscanf(input, "p=<%d,%d,%d>,v=<%d,%d,%d>,a=<%d,%d,%d>",
		&p.pos.x, &p.pos.y, &p.pos.z,
		&p.vel.x, &p.vel.y, &p.vel.z,
		&p.acc.x, &p.acc.y, &p.acc.z)
	if err != nil {
		panic(err)
	}

	return
}

func (p *Particle) tick() {
	p.vel.add(p.acc)
	p.pos.add(p.vel)
}

func (p *Particle) distanceFromOrigin() int {
	return abs(p.pos.x) + abs(p.pos.y) + abs(p.pos.z)
}

func abs(value int) int {
	if value > 0 {
		return value
	}
	return -value
}
