package day16

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 16, "../")

	assert.Equal(t, "kpfonjglcibaedhm", SolvePart1(input))
	assert.Equal(t, "odiabmplhfgjcekn", SolvePart2(input))
}
