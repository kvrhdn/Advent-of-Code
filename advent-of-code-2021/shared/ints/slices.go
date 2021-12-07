package ints

import (
	"math"
	"strconv"
	"strings"
)

func ParseLines(s string) ([]int, error) {
	return Parse(s, func(s string) []string {
		return strings.Split(s, "\n")
	})
}

func Parse(s string, splitFn func(string) []string) ([]int, error) {
	var ints []int
	for _, line := range splitFn(s) {
		i, err := strconv.Atoi(line)
		if err != nil {
			return nil, err
		}
		ints = append(ints, i)
	}
	return ints, nil
}

func MinMax(ints []int) (int, int) {
	min := math.MaxInt
	max := math.MinInt
	for _, i := range ints {
		if i < min {
			min = i
		}
		if i > max {
			max = i
		}
	}
	return min, max
}

func Sum(ints []int) int {
	var sum int
	for _, i := range ints {
		sum += i
	}
	return sum
}

// Windows iterates over a slice in overlapping subslices of length size.
func Windows(ints []int, size int, fn func([]int)) {
	for i := range ints[:len(ints)-(size-1)] {
		fn(ints[i : i+size])
	}
}
