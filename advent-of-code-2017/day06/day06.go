package day06

import (
	"fmt"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/shared/intslice"
)

func SolvePart1(input string) interface{} {
	memoryBank := parseInput(input)

	cycles, _ := redistributeMemoryBankUntilLoop(memoryBank)

	return cycles
}

func SolvePart2(input string) interface{} {
	memoryBank := parseInput(input)

	_, loopSize := redistributeMemoryBankUntilLoop(memoryBank)

	return loopSize
}

func parseInput(input string) []int {
	return intslice.Atoi(strings.Fields(input))
}

func redistributeMemoryBankUntilLoop(memoryBank []int) (cycles, loopSize int) {
	// keep track of previous distributions and in which cycle they occurred
	prevDistributions := make(map[string]int)

	for {
		redistributeMemoryBank(memoryBank)
		cycles += 1

		// not very efficient, but works fine
		hash := fmt.Sprintf("%v", memoryBank)

		// check if this distribution has occurred before
		if prevOccurrence, ok := prevDistributions[hash]; ok {
			return cycles, cycles - prevOccurrence
		}

		prevDistributions[hash] = cycles
	}
}

func redistributeMemoryBank(memoryBank []int) {
	// find largest bank
	largestBank := 0
	largestValue := 0

	for bank, value := range memoryBank {
		if value > largestValue {
			largestBank = bank
			largestValue = value
		}
	}

	// redistribute
	memoryBank[largestBank] = 0

	toRedistribute := largestValue
	curr := (largestBank + 1) % len(memoryBank)

	for {
		memoryBank[curr] += 1
		toRedistribute -= 1

		if toRedistribute == 0 {
			return
		}

		curr = (curr + 1) % len(memoryBank)
	}
}
