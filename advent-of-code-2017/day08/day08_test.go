package day08

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestExample(t *testing.T) {
	input := `b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10`

	assert.Equal(t, 1, SolvePart1(input))
	assert.Equal(t, 10, SolvePart2(input))
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 8, "../")

	assert.Equal(t, 5966, SolvePart1(input))
	assert.Equal(t, 6347, SolvePart2(input))
}
