package day09

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestExamplesPart1(t *testing.T) {
	var cases = []struct {
		input    string
		expected int
	}{
		{"{}", 1},
		{"{{{}}}", 6},
		{"{{},{}}", 5},
		{"{{{},{},{{}}}}", 16},
		{"{<a>,<a>,<a>,<a>}", 1},
		{"{{<ab>},{<ab>},{<ab>},{<ab>}}", 9},
		{"{{<!!>},{<!!>},{<!!>},{<!!>}}", 9},
		{"{{<a!>},{<a!>},{<a!>},{<ab>}}", 3},
		{"{<{}>}", 1},
	}
	for _, c := range cases {
		assert.Equal(t, c.expected, SolvePart1(c.input))
	}
}

func TestExamplesPart2(t *testing.T) {
	var cases = []struct {
		input    string
		expected int
	}{
		{`<>`, 0},
		{`<random characters>`, 17},
		{`<<<<>`, 3},
		{`<{!>}>`, 2},
		{`<!!>`, 0},
		{`<!!!>>`, 0},
		{`<{o"i!a,<{i<a>`, 10},
	}
	for _, c := range cases {
		assert.Equal(t, c.expected, SolvePart2(c.input))
	}
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 9, "../")

	assert.Equal(t, 14212, SolvePart1(input))
	assert.Equal(t, 6569, SolvePart2(input))
}
