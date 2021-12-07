// Package ints contains functions to work with intergers and slices of integers.
package ints

func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}
