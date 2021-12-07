package day07

import (
	"context"
	"math"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/ints"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

func generator(ctx context.Context, input string) ([]int, error) {
	return ints.Parse(input, func(s string) []string {
		return strings.Split(s, ",")
	})
}

func part1(ctx context.Context, input []int) (interface{}, error) {
	min, max := ints.MinMax(input)

	leastFuel := math.MaxInt
	for i := min; i <= max; i++ {
		fuel := 0
		for _, crab := range input {
			fuel += ints.Abs(crab - i)
		}

		if fuel < leastFuel {
			leastFuel = fuel
		}
	}

	return leastFuel, nil
}

func part2(ctx context.Context, input []int) (interface{}, error) {
	min, max := ints.MinMax(input)

	leastFuel := math.MaxInt
	for i := min; i <= max; i++ {
		fuel := 0
		for _, crab := range input {
			steps := ints.Abs(crab - i)

			for i := 0; i < steps; i++ {
				fuel += i + 1
			}
		}

		if fuel < leastFuel {
			leastFuel = fuel
		}
	}

	return leastFuel, nil
}
