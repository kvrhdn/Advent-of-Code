package day21

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day21/square"
	"github.com/stretchr/testify/assert"
)

func TestExamplePart1(t *testing.T) {
	exampleInput := `../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#`

	patterns := parseInput(exampleInput)
	grid := square.ParseGrid(initialGrid)

	grid = enhance(grid, patterns)

	expected := square.ParseGrid("#..#/..../..../#..#")
	assert.True(t, square.Equals(expected, grid))

	grid = enhance(grid, patterns)

	expected = square.ParseGrid("##.##./#..#../....../##.##./#..#../......")
	assert.True(t, square.Equals(expected, grid))
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 21, "../")

	assert.Equal(t, 125, SolvePart1(input))
	assert.Equal(t, 1782917, SolvePart2(input))
}
