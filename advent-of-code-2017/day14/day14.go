package day14

import (
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day14/grid"
)

func SolvePart1(input string) interface{} {
	g := grid.InitGridKnotHash(input)
	return g.SquaresOccupied()
}

func SolvePart2(input string) interface{} {
	g := grid.InitGridKnotHash(input)
	return g.Regionalize()
}
