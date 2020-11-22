package day11

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestExamplesPart1(t *testing.T) {
	cases := []struct {
		input    string
		expected float64
	}{
		{"ne,ne,ne", 3},
		{"ne,ne,sw,sw", 0},
		{"ne,ne,s,s", 2},
		{"se,sw,se,sw,sw", 3},
	}
	for _, c := range cases {
		assert.Equal(t, c.expected, SolvePart1(c.input), "SolvePart1(%v)", c.input)
	}
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 11, "../")

	assert.Equal(t, 834.0, SolvePart1(input))
	assert.Equal(t, 1569.0, SolvePart2(input))
}
