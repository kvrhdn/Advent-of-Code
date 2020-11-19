package day03

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestExamplesPart1(t *testing.T) {
	cases := []struct {
		input    string
		expected int
	}{
		{"1", 0},
		{"12", 3},
		{"23", 2},
		{"1024", 31},
	}
	for _, c := range cases {
		assert.Equal(t, c.expected, SolvePart1(c.input), "SolvePart1(%v)", c.input)
	}
}

func TestExamplesPart2(t *testing.T) {
	cases := []struct {
		input    string
		expected int
	}{
		{"2", 4},
		{"10", 11},
		{"55", 57},
		{"150", 304},
		{"750", 806},
	}
	for _, c := range cases {
		assert.Equal(t, c.expected, SolvePart2(c.input), "SolvePart2(%v)", c.input)
	}
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 3, "../")

	assert.Equal(t, 475, SolvePart1(input))
	assert.Equal(t, 279138, SolvePart2(input))
}
