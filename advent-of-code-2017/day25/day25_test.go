package day25

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 25, "../")

	assert.Equal(t, 2474, SolvePart1(input))
}
