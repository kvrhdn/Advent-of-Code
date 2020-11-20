package day06

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestRedistributeMemoryBank(t *testing.T) {
	memoryBank := []int{0, 2, 7, 0}

	redistributeMemoryBank(memoryBank)

	assert.Equal(t, []int{2, 4, 1, 2}, memoryBank)
}

func TestExample(t *testing.T) {
	input := []int{0, 2, 7, 0}

	cycles, loopSize := redistributeMemoryBankUntilLoop(input)

	assert.Equal(t, 5, cycles)
	assert.Equal(t, 4, loopSize)
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 6, "../")

	assert.Equal(t, 7864, SolvePart1(input))
	assert.Equal(t, 1695, SolvePart2(input))
}
