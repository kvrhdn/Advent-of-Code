package day24

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestExamplePart1(t *testing.T) {
	exampleInput := `0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10`

	assert.Equal(t, 31, SolvePart1(exampleInput))
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 24, "../")

	assert.Equal(t, 1868, SolvePart1(input))
	assert.Equal(t, 1841, SolvePart2(input))
}
