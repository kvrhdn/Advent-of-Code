package particle

import (
	"fmt"
	"strings"
)

type Particle struct {
	ID      int
	A, V, P Vec3
}

func ParseParticle(id int, input string) (p Particle) {
	p.ID = id

	input = strings.ReplaceAll(input, " ", "")

	_, err := fmt.Sscanf(input, "p=<%d,%d,%d>,v=<%d,%d,%d>,a=<%d,%d,%d>",
		&p.P.X, &p.P.Y, &p.P.Z,
		&p.V.X, &p.V.Y, &p.V.Z,
		&p.A.X, &p.A.Y, &p.A.Z)
	if err != nil {
		panic(err)
	}

	return
}

func (p *Particle) String() string {
	return fmt.Sprintf("Particle %v: a %v, v %v, p %v", p.ID, p.A, p.V, p.P)
}

func (p *Particle) Tick() {
	p.V.Add(p.A)
	p.P.Add(p.V)
}

func (p *Particle) DistanceOrigin() int {
	return abs(p.P.X) + abs(p.P.Y) + abs(p.P.Z)
}

func abs(value int) int {
	if value > 0 {
		return value
	}
	return -1 * value
}
