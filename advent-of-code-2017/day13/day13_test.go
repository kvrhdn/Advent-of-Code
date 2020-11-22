package day13

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestExample(t *testing.T) {
	example := `0: 3
1: 2
4: 4
6: 4`

	assert.Equal(t, 24, SolvePart1(example))
	assert.Equal(t, 10, SolvePart2(example))
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 13, "../")

	assert.Equal(t, 1728, SolvePart1(input))
	assert.Equal(t, 3946838, SolvePart2(input))
}
