package day11

import (
	"math"
	"strings"
)

func SolvePart1(input string) interface{} {
	steps := strings.Split(input, ",")

	pos := origin()
	for _, s := range steps {
		pos.step(s)
	}

	return pos.distanceFromOrigin()
}

func SolvePart2(input string) interface{} {
	steps := strings.Split(input, ",")

	furthestDistance := 0.0

	pos := origin()
	for _, s := range steps {
		pos.step(s)

		distance := pos.distanceFromOrigin()
		if distance > furthestDistance {
			furthestDistance = distance
		}
	}

	return furthestDistance
}

type Pos struct {
	northSouth float64
	eastWest   float64
}

func origin() Pos {
	return Pos{}
}

func (p *Pos) distanceFromOrigin() float64 {
	return math.Abs(p.northSouth) + math.Abs(p.eastWest)
}

func (p *Pos) step(step string) {
	value := 1 / float64(len(step))

	if strings.Contains(step, "n") {
		p.northSouth += value
	}
	if strings.Contains(step, "s") {
		p.northSouth -= value
	}
	if strings.Contains(step, "e") {
		p.eastWest += value
	}
	if strings.Contains(step, "w") {
		p.eastWest -= value
	}
}
