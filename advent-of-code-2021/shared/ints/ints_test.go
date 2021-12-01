package ints

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParse(t *testing.T) {
	tests := []struct {
		str    string
		output []int
		err    bool
	}{
		{
			str:    "123\n456\n0\n-123",
			output: []int{123, 456, 0, -123},
		},
		{
			str: "123\n\n456",
			err: true,
		},
		{
			str: "123\n0.1",
			err: true,
		},
	}
	for _, test := range tests {
		t.Run(test.str, func(t *testing.T) {
			output, err := Parse(test.str)

			if test.err {
				assert.Error(t, err)
			} else {
				assert.NoError(t, err)
				assert.Equal(t, output, test.output)
			}
		})
	}
}

func TestSum(t *testing.T) {
	tests := []struct {
		ints []int
		sum  int
	}{
		{
			[]int{1, 2, 3},
			6,
		},
		{
			[]int{1, 2, 3, -12},
			-6,
		},
	}
	for _, test := range tests {
		t.Run(fmt.Sprintf("%v", test.ints), func(t *testing.T) {
			assert.Equal(t, test.sum, Sum(test.ints))
		})
	}
}

func TestWindows_Size2(t *testing.T) {
	invocationCount := 0

	Windows([]int{1, 2, 3, 4}, 2, func(window []int) {
		switch invocationCount {
		case 0:
			assert.Equal(t, []int{1, 2}, window)
		case 1:
			assert.Equal(t, []int{2, 3}, window)
		case 2:
			assert.Equal(t, []int{3, 4}, window)
		default:
			t.Errorf("unexpected invocationCount %d", invocationCount)
		}

		invocationCount++
	})
}

func TestWindows_Size3(t *testing.T) {
	invocationCount := 0

	Windows([]int{1, 2, 3, 4}, 3, func(window []int) {
		switch invocationCount {
		case 0:
			assert.Equal(t, []int{1, 2, 3}, window)
		case 1:
			assert.Equal(t, []int{2, 3, 4}, window)
		default:
			t.Errorf("unexpected invocationCount %d", invocationCount)
		}

		invocationCount++
	})
}
