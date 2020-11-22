package day12

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestExample(t *testing.T) {
	var input = `0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5`

	assert.Equal(t, 6, SolvePart1(input))
	assert.Equal(t, 2, SolvePart2(input))
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 12, "../")

	assert.Equal(t, 128, SolvePart1(input))
	assert.Equal(t, 209, SolvePart2(input))
}
