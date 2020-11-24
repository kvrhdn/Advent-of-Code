package day22

import (
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day22/grid"
)

const (
	Clean    = 0 // corresponds to '.'
	Infected = 1 // corresponds to '#'
	Weakened = 2
	Flagged  = 3
)

type VirusCarrier struct {
	pos       grid.Pos
	dir       grid.Dir
	virusImpl func(*VirusCarrier, int) int
}

func (c *VirusCarrier) process(current int) (new int) {
	return c.virusImpl(c, current)
}

func (c *VirusCarrier) step() {
	c.pos = grid.Step(c.pos, c.dir)
}

func virusV1(c *VirusCarrier, current int) (result int) {
	switch current {
	case Clean:
		c.dir = grid.LeftOf(c.dir)
		result = Infected

	case Infected:
		c.dir = grid.RightOf(c.dir)
		result = Clean
	}
	return
}

func virusV2(c *VirusCarrier, current int) (result int) {
	switch current {
	case Clean:
		c.dir = grid.LeftOf(c.dir)
		result = Weakened

	case Weakened:
		result = Infected

	case Infected:
		c.dir = grid.RightOf(c.dir)
		result = Flagged

	case Flagged:
		c.dir = grid.ReverseOf(c.dir)
		result = Clean
	}
	return
}
