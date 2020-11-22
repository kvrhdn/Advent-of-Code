package knotHash

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestExampleDay10Part1(t *testing.T) {
	size := 5
	lengths := []int{3, 4, 1, 5}

	knot := newWithCustomSize(size, lengths)

	knot.DoARound()

	assert.Equal(t, []int{3, 4, 2, 1, 0}, knot.Numbers)
	assert.Equal(t, 4, knot.position%size)
	assert.Equal(t, 4, knot.skipSize)
}

func TestExamplesDay10Part2(t *testing.T) {
	cases := []struct {
		input, expected string
	}{
		{"", "a2582a3a0e66e6e86e3812dcb672a272"},
		{"AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd"},
		{"1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d"},
		{"1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e"},
	}
	for _, c := range cases {
		assert.Equal(t, c.expected, DenseKnotHash(c.input), "DenseKnotHash(%v)", c.input)
	}
}

func TestNewDenseKnotHash(t *testing.T) {
	assert.Equal(
		t,
		[]int{49, 44, 50, 44, 51, 17, 31, 73, 47, 23},
		newDenseKnotHash("1,2,3").lengths,
	)
}
