package day15

import (
	"fmt"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/shared/util"
)

const (
	factorA = 16807
	factorB = 48271

	million = 1000000
)

func SolvePart1(input string) interface{} {
	valueA, valueB := parseInput(input)

	count := 0

	util.Times(40*million, func() {
		valueA = generateNext(valueA, factorA)
		valueB = generateNext(valueB, factorB)

		if (valueA & 0xFFFF) == (valueB & 0xFFFF) {
			count++
		}
	})

	return count
}

func SolvePart2(input string) interface{} {
	valueA, valueB := parseInput(input)

	count := 0

	util.Times(5*million, func() {
		valueA = generateNextMultipleOf(valueA, factorA, 4)
		valueB = generateNextMultipleOf(valueB, factorB, 8)

		if (valueA & 0xFFFF) == (valueB & 0xFFFF) {
			count++
		}
	})

	return count
}

func parseInput(input string) (valueA, valueB int) {
	_, err := fmt.Sscanf(input, "Generator A starts with %d\nGenerator B starts with %d", &valueA, &valueB)
	if err != nil {
		panic(err)
	}
	return
}

func generateNext(value, factor int) int {
	return (value * factor) % 2147483647
}

func generateNextMultipleOf(value, factor, dividableBy int) int {
	for {
		value = generateNext(value, factor)

		if value%dividableBy == 0 {
			return value
		}
	}
}
