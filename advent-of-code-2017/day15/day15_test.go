package day15

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestGenerateNext(t *testing.T) {
	var cases = []struct {
		startValue int
		factor     int
		expected   []int
	}{
		{65, factorA, []int{1092455, 1181022009, 245556042, 1744312007, 1352636452}},
		{8921, factorB, []int{430625591, 1233683848, 1431495498, 137874439, 285222916}},
	}
	for _, c := range cases {
		value := c.startValue

		for _, expected := range c.expected {
			value = generateNext(value, c.factor)
			assert.Equal(t, expected, value)
		}
	}
}

func TestGenerateNextMultipleOf(t *testing.T) {
	var cases = []struct {
		startValue int
		factor     int
		mulitpleOf int
		expected   []int
	}{
		{65, factorA, 4, []int{1352636452, 1992081072, 530830436, 1980017072, 740335192}},
		{8921, factorB, 8, []int{1233683848, 862516352, 1159784568, 1616057672, 412269392}},
	}
	for _, c := range cases {
		value := c.startValue

		for _, expected := range c.expected {
			value = generateNextMultipleOf(value, c.factor, c.mulitpleOf)
			assert.Equal(t, expected, value)
		}
	}
}

func TestExample(t *testing.T) {
	example := `Generator A starts with 65
Generator B starts with 8921`

	assert.Equal(t, 588, SolvePart1(example))
	assert.Equal(t, 309, SolvePart2(example))
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 15, "../")

	assert.Equal(t, 569, SolvePart1(input))
	assert.Equal(t, 298, SolvePart2(input))
}
