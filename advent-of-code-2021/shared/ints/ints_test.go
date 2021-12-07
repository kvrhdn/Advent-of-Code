package ints

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestAbs(t *testing.T) {
	tests := []struct {
		value, expected int
	}{
		{1, 1},
		{-1, 1},
		{0, 0},
	}
	for _, tt := range tests {
		t.Run(strconv.Itoa(tt.value), func(t *testing.T) {
			assert.Equal(t, tt.expected, Abs(tt.value))
		})
	}
}
