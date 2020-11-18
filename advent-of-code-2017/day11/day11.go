package day11

import (
	"math"
	"strings"
)

func SolvePart1(input string) interface{} {
	steps := strings.Split(input, ",")

	pos := Pos{}
	pos.TakeSteps(steps)

	return pos.DistanceFromOrigin()
}

func SolvePart2(input string) interface{} {
	steps := strings.Split(input, ",")

	pos := Pos{}

	largestDistance := 0.0

	for _, step := range steps {
		pos.TakeStep(step)

		distance := pos.DistanceFromOrigin()
		if distance > largestDistance {
			largestDistance = distance
		}
	}

	return largestDistance
}

type Pos struct {
	northSouth float64
	eastWest   float64
}

func (p *Pos) DistanceFromOrigin() float64 {
	return math.Abs(p.northSouth) + math.Abs(p.eastWest)
}

func (p *Pos) TakeSteps(steps []string) {
	for _, step := range steps {
		p.TakeStep(step)
	}
}

func (p *Pos) TakeStep(step string) {
	value := 1 / float64(len(step))

	if strings.ContainsAny(step, "n") {
		p.northSouth += value
	}
	if strings.ContainsAny(step, "s") {
		p.northSouth -= value
	}
	if strings.ContainsAny(step, "e") {
		p.eastWest += value
	}
	if strings.ContainsAny(step, "w") {
		p.eastWest -= value
	}
}
