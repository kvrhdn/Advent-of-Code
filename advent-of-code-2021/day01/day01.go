package day01

import (
	"context"
	"fmt"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
)

var Solution = aoc.NewDayGen(generator, part1, nil)

func generator(ctx context.Context, input string) (int, error) {
	return len(input), nil
}

func part1(ctx context.Context, input int) (string, error) {
	return fmt.Sprint(input), nil
}
