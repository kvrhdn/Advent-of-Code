package day10

import (
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day10/knotHash"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/shared/intslice"
)

func SolvePart1(input string) interface{} {
	lengths := intslice.Atoi(strings.Split(input, ","))

	numbers := knotHash.CreateNumbersUpTo(255)

	knotHash.KnotHashRound(numbers, lengths, 0, 0)

	return numbers[0] * numbers[1]
}

func SolvePart2(input string) interface{} {
	denseKnotHash := knotHash.DenseKnotHash(input)

	return denseKnotHash
}
