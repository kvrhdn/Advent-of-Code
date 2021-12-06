package day06

import (
	"context"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/ints"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/slices"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

func generator(ctx context.Context, input string) ([]int, error) {
	return ints.Parse(input, func(s string) []string {
		return strings.Split(s, ",")
	})
}

func part1(ctx context.Context, input []int) (interface{}, error) {
	lanternFish := slices.Copy(input)

	day := 0
	for day < 80 {
		day += 1

		len := len(lanternFish)
		for i := 0; i < len; i++ {
			if lanternFish[i] == 0 {
				lanternFish[i] = 6
				lanternFish = append(lanternFish, 8)
				continue
			}

			lanternFish[i] -= 1
		}
	}

	return len(lanternFish), nil
}

func part2(ctx context.Context, input []int) (interface{}, error) {
	lanternFish := make(map[int]int)

	for _, fish := range input {
		lanternFish[fish] += 1
	}

	day := 0
	for day < 256 {
		day += 1

		newLanternFish := make(map[int]int)

		// handle fish at 0
		newLanternFish[8] += lanternFish[0]
		newLanternFish[6] += lanternFish[0]

		// handle all other fish
		for i := 1; i <= 8; i++ {
			newLanternFish[i-1] += lanternFish[i]
		}

		lanternFish = newLanternFish
	}

	count := 0
	for _, c := range lanternFish {
		count += c
	}

	return count, nil
}
