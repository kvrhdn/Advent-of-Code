package day01

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
		{"1122", 3},
		{"1111", 4},
		{"1234", 0},
		{"91212129", 9},
	}
	for _, c := range cases {
		assert.Equal(t, c.expected, SolvePart1(c.input), "SolvePart1(%s)", c.input)
	}
}

func TestExamplesPart2(t *testing.T) {
	cases := []struct {
		input    string
		expected int
	}{
		{"1212", 6},
		{"1221", 0},
		{"123425", 4},
		{"123123", 12},
		{"12131415", 4},
	}
	for _, c := range cases {
		assert.Equal(t, c.expected, SolvePart2(c.input), "SolvePart2(%s)", c.input)
	}
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 1, "../")

	assert.Equal(t, 1175, SolvePart1(input))
	assert.Equal(t, 1166, SolvePart2(input))
}
