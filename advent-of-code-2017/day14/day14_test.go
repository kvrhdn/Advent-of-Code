package day14

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestExample(t *testing.T) {
	exampleInput := "flqrgnkx"

	assert.Equal(t, 8108, SolvePart1(exampleInput))
	assert.Equal(t, 1242, SolvePart2(exampleInput))
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 14, "../")

	assert.Equal(t, 8226, SolvePart1(input))
	assert.Equal(t, 1128, SolvePart2(input))
}
