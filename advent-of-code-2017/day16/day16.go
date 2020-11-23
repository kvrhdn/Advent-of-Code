package day16

import (
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/shared/util"
)

const (
	initialPrograms = "abcdefghijklmnop"
	billion         = 1000000000
)

func SolvePart1(input string) interface{} {
	moves := parseInput(input)
	programs := []rune(initialPrograms)

	for _, move := range moves {
		move.apply(&programs)
	}

	return string(programs)
}

func SolvePart2(input string) interface{} {
	moves := parseInput(input)

	offset, period := findRepetition(initialPrograms, func(value string) string {
		p := []rune(value)

		for _, move := range moves {
			move.apply(&p)
		}

		return string(p)
	})

	iterationWanted := (billion - offset) % period

	programs := []rune(initialPrograms)

	util.Times(iterationWanted, func() {
		for _, move := range moves {
			move.apply(&programs)
		}
	})

	return string(programs)
}

func parseInput(input string) (moves []DanceMove) {
	for _, line := range strings.Split(input, ",") {
		moves = append(moves, parseDanceMove(line))
	}
	return
}

func findRepetition(initialValue string, process func(string) string) (offset, period int) {
	prevStates := make(map[string]int)
	value := initialValue

	iteration := 0

	prevStates[value] = iteration

	for {
		value = process(value)
		iteration++

		prevIteration, ok := prevStates[value]
		if ok {
			return prevIteration, iteration
		}

		prevStates[value] = iteration
	}
}
