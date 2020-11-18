package day16

import (
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day16/dance"
)

func SolvePart1(input string) interface{} {
	danceMoves := dance.ParseMoves(input)

	doTheDance := func(programs string) string {
		return dance.Dance(programs, danceMoves)
	}

	return doTheDance(initialPrograms)
}

func SolvePart2(input string) interface{} {
	danceMoves := dance.ParseMoves(input)

	doTheDance := func(programs string) string {
		return dance.Dance(programs, danceMoves)
	}

	offset, period := findRepetition(initialPrograms, doTheDance)
	iterationWanted := (one_billion - offset) % period

	return executeTimes(initialPrograms, doTheDance, iterationWanted)
}

const initialPrograms = "abcdefghijklmnop"
const one_billion = 1000000000

func findRepetition(input string, process func(string) string) (offset, period int) {
	occurrences := make(map[string]int)
	value := input

	iteration := 0

	occurrences[value] = iteration

	for {
		value = process(value)
		iteration += 1

		prevIteration, ok := occurrences[value]
		if ok {
			return prevIteration, iteration
		}

		occurrences[value] = iteration
	}
}

func executeTimes(input string, process func(string) string, times int) string {
	value := input

	for i := 0; i < times; i++ {
		value = process(value)
	}

	return value
}
