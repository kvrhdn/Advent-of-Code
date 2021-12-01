package day01

import (
	"context"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/ints"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

func generator(ctx context.Context, input string) ([]int, error) {
	return ints.Parse(input)
}

func part1(ctx context.Context, input []int) (interface{}, error) {
	increases := 0

	prevDepth := input[0]
	for _, depth := range input[1:] {
		if depth > prevDepth {
			increases += 1
		}
		prevDepth = depth
	}

	return increases, nil
}

func part2(ctx context.Context, input []int) (interface{}, error) {
	increases := 0

	prevDepth := ints.Sum(input[0:3])
	ints.Windows(input[1:], 3, func(window []int) {
		depth := ints.Sum(window)
		if depth > prevDepth {
			increases += 1
		}
		prevDepth = depth
	})

	return increases, nil
}
