package day22

import (
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day22/grid"
)

func SolvePart1(input string) interface{} {
	g := parseInput(input)
	c := &VirusCarrier{
		pos:       g.Center(),
		dir:       grid.North,
		virusImpl: virusV1,
	}

	return infectionsAfterBursts(10_000, g, c)
}

func SolvePart2(input string) interface{} {
	g := parseInput(input)
	c := &VirusCarrier{
		pos:       g.Center(),
		dir:       grid.North,
		virusImpl: virusV2,
	}

	return infectionsAfterBursts(10_000_000, g, c)
}

func parseInput(input string) grid.InfiniteGrid {
	infGrid := grid.New()

	for y, l := range strings.Split(input, "\n") {
		for x, r := range l {
			// we have to set all cells to be able to calculate the center later
			if r == '#' {
				infGrid.Set(grid.NewPos(x, y), Infected)
			} else {
				infGrid.Set(grid.NewPos(x, y), Clean)
			}
		}
	}

	return infGrid
}

func infectionsAfterBursts(bursts int, g grid.InfiniteGrid, c *VirusCarrier) (infections int) {
	for i := 0; i < bursts; i++ {
		newState := c.process(g.Get(c.pos))

		g.Set(c.pos, newState)
		if newState == Infected {
			infections++
		}

		c.step()
	}
	return
}
