package day01

import (
	"context"
	"strconv"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

func generator(ctx context.Context, input string) (ints []int, err error) {
	for _, line := range strings.Split(input, "\n") {
		i, err := strconv.Atoi(line)
		if err != nil {
			return nil, err
		}
		ints = append(ints, i)
	}
	return
}

func part1(ctx context.Context, input []int) (string, error) {
	increases := 0

	prevDepth := input[0]
	for _, depth := range input[1:] {
		if depth > prevDepth {
			increases += 1
		}
		prevDepth = depth
	}

	return strconv.Itoa(increases), nil
}

func part2(ctx context.Context, input []int) (string, error) {
	increases := 0

	prevDepth := sum(input[0:3])
	for i := range input[1 : len(input)-1] {
		depth := sum(input[i : i+3])
		if depth > prevDepth {
			increases += 1
		}
		prevDepth = depth
	}

	return strconv.Itoa(increases), nil
}

func sum(slice []int) (sum int) {
	for _, s := range slice {
		sum += s
	}
	return
}
