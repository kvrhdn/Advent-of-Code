package day10

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 10, "../")

	assert.Equal(t, 8536, SolvePart1(input))
	assert.Equal(t, "aff593797989d665349efe11bb4fd99b", SolvePart2(input))
}
