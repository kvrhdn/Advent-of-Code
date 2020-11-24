package day21

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day21/square"
	"github.com/stretchr/testify/assert"
)

func TestPatternMatcher2(t *testing.T) {
	grid := square.Grid([][]rune{
		{'#', '.', '.', '#'},
		{'.', '.', '.', '.'},
		{'.', '.', '.', '.'},
		{'#', '.', '.', '#'},
	})

	patternMatcher := parsePattern("../.# => ../..")

	// should match
	assert.True(t, patternMatcher.matches(square.Slice(grid, 0, 0, 2)))
	assert.True(t, patternMatcher.matches(square.Slice(grid, 2, 0, 2)))
	assert.True(t, patternMatcher.matches(square.Slice(grid, 0, 2, 2)))
	assert.True(t, patternMatcher.matches(square.Slice(grid, 2, 2, 2)))

	// should not match
	assert.False(t, patternMatcher.matches(square.Slice(grid, 0, 0, 3)))
	assert.False(t, patternMatcher.matches(square.Slice(grid, 1, 0, 2)))
	assert.False(t, patternMatcher.matches(square.Slice(grid, 1, 1, 2)))
	assert.False(t, patternMatcher.matches(square.Slice(grid, 1, 2, 2)))
}

func TestPatternMatcher3(t *testing.T) {
	patternMatcher := parsePattern(".#./..#/### => ../..")

	cases := []string{
		".#./..#/###",
		".#./#../###",
		"#../#.#/##.",
		"###/..#/.#.",
	}
	for _, c := range cases {
		grid := square.ParseGrid(c)

		assert.True(t, patternMatcher.matches(grid), c)
	}
}
