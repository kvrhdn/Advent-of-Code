package day06

import (
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/util"
)

func SolvePart1(input string) interface{} {
	memoryBank := initMemoryBankFrom(input)

	cycles, _ := RedistributeCyclesUntilLoop(memoryBank)

	return cycles
}

func SolvePart2(input string) interface{} {
	memoryBank := initMemoryBankFrom(input)

	_, loopSize := RedistributeCyclesUntilLoop(memoryBank)

	return loopSize
}

func initMemoryBankFrom(input string) []int {
	splitInput := strings.Fields(input)

	return util.SliceAtoi(splitInput)
}

func RedistributeCyclesUntilLoop(memoryBank []int) (cycles, loopSize int) {
	var prevBanks [][]int

	prevBanks = append(prevBanks, util.SliceCopy(memoryBank))

	for {
		redistributeMemoryBank(memoryBank)
		cycles += 1

		prevBanks = append(prevBanks, util.SliceCopy(memoryBank))

		if containsDuplicate(prevBanks) {
			break
		}
	}

	for i := len(prevBanks) - 2; i > 0; i-- {
		if util.SliceEquals(prevBanks[i], prevBanks[len(prevBanks)-1]) {
			return cycles, len(prevBanks) - 1 - i
		}
	}

	panic("should not have happened D:")
}

func containsDuplicate(memoryBanks [][]int) bool {
	for i, bank1 := range memoryBanks {
		for _, bank2 := range memoryBanks[i+1:] {
			if util.SliceEquals(bank1, bank2) {
				return true
			}
		}
	}
	return false
}

func redistributeMemoryBank(memoryBank []int) {
	maxBank := 0
	maxValue := 0

	for bank, value := range memoryBank {
		if value > maxValue {
			maxBank = bank
			maxValue = value
		}
	}

	banks := len(memoryBank)
	bank := incCircular(maxBank, banks)

	memoryBank[maxBank] = 0

	for values := maxValue; values > 0; values-- {
		memoryBank[bank] += 1

		bank = incCircular(bank, banks)
	}
}

func incCircular(index, indexRange int) int {
	return (index + 1) % indexRange
}
