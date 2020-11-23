package day17

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestExamplePart1(t *testing.T) {
	assert.Equal(t, 638, SolvePart1("3"))
}

func TestSpinlockAlgorithmFindValueAfter0(t *testing.T) {
	steps := 3
	expected := 5
	stopAt := 5

	// 0
	// 0 1
	// 0 2 1
	// 0 2 3 1
	// 0 2 4 3 1
	// 0 5 2 4 3 1

	assert.Equal(t, expected, spinlockAlgorithmFindValueAfter0(steps, stopAt))
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 17, "../")

	assert.Equal(t, 808, SolvePart1(input))
	assert.Equal(t, 47465686, SolvePart2(input))
}
