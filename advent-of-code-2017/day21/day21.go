package day21

import (
	"fmt"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day21/square"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/shared/util"
)

const initialGrid = ".#./..#/###"

func SolvePart1(input string) interface{} {
	patterns := parseInput(input)
	grid := square.ParseGrid(initialGrid)

	fmt.Printf("%v\n", grid)

	util.Times(5, func() {
		grid = enhance(grid, patterns)
		fmt.Printf("Enhance!\n\n%v\n", grid)
	})

	return square.Count(grid, '#')
}

func SolvePart2(input string) interface{} {
	patterns := parseInput(input)
	grid := square.ParseGrid(initialGrid)

	util.Times(18, func() {
		grid = enhance(grid, patterns)
	})

	return square.Count(grid, '#')
}

func parseInput(input string) (patterns []PatternMatcher) {
	for _, line := range strings.Split(input, "\n") {
		patterns = append(patterns, parsePattern(line))
	}
	return
}

func enhance(g square.Grid, patterns []PatternMatcher) square.Grid {
	var blockWidth int

	if g.Size()%2 == 0 {
		blockWidth = 2
	} else if g.Size()%3 == 0 {
		blockWidth = 3
	} else {
		panic("grid size should be dividable by either 2 or 3")
	}
	blockCount := g.Size() / blockWidth

	blocks := square.Divide(g, blockWidth)

	var outputs []square.GridLike
	for _, slice := range blocks {
		outputs = append(outputs, replace(slice, patterns))
	}

	return square.Combine(outputs, blockCount)
}

func replace(g square.GridLike, patterns []PatternMatcher) square.Grid {
	for _, p := range patterns {
		if p.matches(g) {
			return p.output
		}
	}
	panic("no pattern matched with block")
}
