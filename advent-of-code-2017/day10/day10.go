package day10

import (
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/shared/intslice"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/shared/knotHash"
)

func SolvePart1(input string) interface{} {
	lengths := intslice.Atoi(strings.Split(input, ","))

	knot := knotHash.New(lengths)
	knot.DoARound()

	return knot.Numbers[0] * knot.Numbers[1]
}

func SolvePart2(input string) interface{} {
	return knotHash.DenseKnotHash(input)
}
