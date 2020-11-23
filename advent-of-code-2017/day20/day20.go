package day20

import (
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day20/particle"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/shared/util"
)

func SolvePart1(input string) interface{} {
	w := particle.ParseWorld(input)

	// update the world a bunch of times until the closest remaining is actually the closest particle
	util.Times(500, func() {
		w.Tick()
	})

	return w.FindClosestToOrigin().ID
}

func SolvePart2(input string) interface{} {
	w := particle.ParseWorld(input)

	// update the world a bunch of times until all collisions have occurred
	util.Times(100, func() {
		w.Tick()
		w.RemoveCollisions()
	})

	return len(w)
}
