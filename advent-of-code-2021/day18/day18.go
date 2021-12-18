package day18

import (
	"context"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day18/snailfish"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

func generator(ctx context.Context, input string) ([]string, error) {
	var numbers []string
	for _, line := range strings.Split(input, "\n") {
		numbers = append(numbers, line)
	}
	return numbers, nil
}

func part1(ctx context.Context, input []string) (interface{}, error) {
	return snailfish.Add(input...).Magnitude(), nil
}

func part2(ctx context.Context, input []string) (interface{}, error) {
	largestMagnitude := 0

	// loop over the raw strings and just keep reparsing them because snailfish
	// isn't a well-written library and modifies numbers when adding them...
	for _, n1 := range input {
		for _, n2 := range input {
			if n1 == n2 {
				continue
			}

			magnitude := snailfish.Add(n1, n2).Magnitude()

			if magnitude > largestMagnitude {
				largestMagnitude = magnitude
			}
		}
	}

	return largestMagnitude, nil
}
