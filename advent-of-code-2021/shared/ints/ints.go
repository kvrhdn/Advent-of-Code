// Package ints contains functions to work with slices of integers.
package ints

import (
	"strconv"
	"strings"
)

func Parse(s string) ([]int, error) {
	var ints []int
	for _, line := range strings.Split(s, "\n") {
		i, err := strconv.Atoi(line)
		if err != nil {
			return nil, err
		}
		ints = append(ints, i)
	}
	return ints, nil
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
