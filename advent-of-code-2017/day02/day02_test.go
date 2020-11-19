package day02

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestParseInput(t *testing.T) {
	cases := []struct {
		input    string
		expected [][]int
	}{
		{"1 2\n3 4", [][]int{{1, 2}, {3, 4}}},
		{"321\n3312 41231 234342", [][]int{{321}, {3312, 41231, 234342}}},
	}
	for _, c := range cases {
		assert.Equal(t, c.expected, parseInput(c.input), "parseInput(%s)", c.input)
	}
}

func TestExamplePart1(t *testing.T) {
	example := `5 1 9 5
7 5 3
2 4 6 8`

	assert.Equal(t, 18, SolvePart1(example))
}

func TestExamplePart2(t *testing.T) {
	example := `5 9 2 8
9 4 7 3
3 8 6 5`

	assert.Equal(t, 9, SolvePart2(example))
}

func TestRangeOf(t *testing.T) {
	cases := []struct {
		input    []int
		expected int
	}{
		{[]int{1, 2, 3}, 2},
		{[]int{4, 1, 3}, 3},
	}
	for _, c := range cases {
		assert.Equal(t, c.expected, rangeOf(c.input), "rangeOf(%v)", c.input)
	}
}

func TestEvenlyDivisbleResult(t *testing.T) {
	cases := []struct {
		input    []int
		expected int
	}{
		{[]int{5, 9, 2, 8}, 4},
		{[]int{9, 4, 7, 3}, 3},
		{[]int{3, 8, 6, 5}, 2},
	}
	for _, c := range cases {
		assert.Equal(t, c.expected, resultOfEvenlyDivisibleNumber(c.input), "resultOfEvenlyDivisibleNumber(%v)", c.input)
	}
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 2, "../")

	assert.Equal(t, 45158, SolvePart1(input))
	assert.Equal(t, 294, SolvePart2(input))
}
