package day05

import (
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/util"
)

func SolvePart1(input string) interface{} {
	instructionsString := strings.Split(input, "\n")
	instructions := util.SliceAtoi(instructionsString)

	return JumpThroughInstructionsUntilOutOfBounds(instructions)
}

func SolvePart2(input string) interface{} {
	instructionsString := strings.Split(input, "\n")
	instructions := util.SliceAtoi(instructionsString)

	return SpecialJumpThroughInstructionsUntilOutOfBounds(instructions)
}

func JumpThroughInstructionsUntilOutOfBounds(instructions []int) int {
	maxIndex := len(instructions)

	steps := 0
	index := 0

	for {
		prevIndex := index

		index += instructions[prevIndex]
		instructions[prevIndex] += 1

		steps++

		if index < 0 || index >= maxIndex {
			return steps
		}
	}
}

func SpecialJumpThroughInstructionsUntilOutOfBounds(instructions []int) int {
	maxIndex := len(instructions)

	steps := 0
	index := 0

	for {
		prevIndex := index

		offset := instructions[prevIndex]

		index += offset

		if offset >= 3 {
			instructions[prevIndex] -= 1
		} else {
			instructions[prevIndex] += 1
		}

		steps++

		if index < 0 || index >= maxIndex {
			return steps
		}
	}
}
