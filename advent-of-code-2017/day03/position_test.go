package day03

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestManhattanDistanceToOrigin(t *testing.T) {
	cases := []struct {
		input    Pos
		expected int
	}{
		{Pos{3, 4}, 7},
		{Pos{-3, 0}, 3},
		{Pos{1, -4}, 5},
	}
	for _, c := range cases {
		assert.Equal(t, c.expected, c.input.ManhattanDistanceToOrigin(), "%v.ManhattanDistanceToOrigin()", c.input)
	}
}
