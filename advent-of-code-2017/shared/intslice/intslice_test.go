package intslice

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestEquals(t *testing.T) {
	cases := []struct {
		input1, input2 []int
		expected       bool
	}{
		{[]int{1, 2, 3}, []int{1, 2, 3}, true},
		{[]int{1, 2, 3}, []int{1, 3}, false},
		{[]int{3, 2, -1}, []int{3, 2, 1}, false},
	}
	for _, c := range cases {
		assert.Equal(t, c.expected, Equals(c.input1, c.input2), "Equals(%v, %v)", c.input1, c.input2)
	}
}

func TestCopy(t *testing.T) {
	original := []int{1, 2, 3}

	copied := Copy(original)

	assert.True(t, Equals(original, copied))

	// make sure manipulating copied doesn't influence the original
	copied[0] = 4

	assert.Equal(t, 1, original[0])
}

func TestAtoi(t *testing.T) {
	input := []string{"123", "4", "-5", "234238"}
	expected := []int{123, 4, -5, 234238}

	result := Atoi(input)

	assert.True(t, Equals(expected, result), "Equals(%v, %v)", expected, result)
}

func TestAtoi_nonDigit(t *testing.T) {
	assert.Panics(t, func() {
		Atoi([]string{"123", "a", "5"})
	})
}
