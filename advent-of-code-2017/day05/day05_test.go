package day05

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestExamplePart1(t *testing.T) {
	input := `0
3
0
1
-3`
	result := SolvePart1(input)

	assert.Equal(t, 5, result)
}

func TestExamplePart2(t *testing.T) {
	input := `0
3
0
1
-3`
	result := SolvePart2(input)

	assert.Equal(t, 10, result)
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 5, "../")

	assert.Equal(t, 354121, SolvePart1(input))
	assert.Equal(t, 27283023, SolvePart2(input))
}
