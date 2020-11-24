package day23

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestDebugMode(t *testing.T) {
	expectedReg := pack(0, 93, 93, 93, 93, 0, 0, 1)

	// emulated coprocessor
	input := aoc.ReadInputRelative(2017, 23, "../")
	instructions := parseInput(input)

	coprocessor := newProcessor()
	coprocessor.execute(instructions)

	assert.Equal(t, 8281, coprocessor.mulCalled)
	assert.Equal(t, expectedReg, coprocessor.reg)

	// rewritten the instructions as Go code
	reg, mulCalled := runDirectAssembly(0)

	assert.Equal(t, 8281, mulCalled)
	assert.Equal(t, expectedReg, reg)

	// optimized solution
	reg = runOptimized(0)

	assert.Equal(t, expectedReg, reg)
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 23, "../")

	assert.Equal(t, 8281, SolvePart1(input))
	assert.Equal(t, 911, SolvePart2(input))
}
