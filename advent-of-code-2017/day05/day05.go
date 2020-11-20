package day05

import (
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/shared/intslice"
)

func SolvePart1(input string) interface{} {
	instructions := parseInput(input)

	return runProgram(instructions, func(ir *int) {
		*ir += 1
	})
}

func SolvePart2(input string) interface{} {
	instructions := parseInput(input)

	return runProgram(instructions, func(ir *int) {
		if *ir >= 3 {
			*ir -= 1
		} else {
			*ir += 1
		}
	})
}

func parseInput(input string) []int {
	return intslice.Atoi(strings.Split(input, "\n"))
}

func runProgram(instructions []int, modifyAfterJump func(*int)) (steps int) {
	curr := 0

	for {
		prev := curr

		curr += instructions[prev]
		modifyAfterJump(&instructions[prev])

		steps++

		if curr < 0 || curr >= len(instructions) {
			return steps
		}
	}
}
