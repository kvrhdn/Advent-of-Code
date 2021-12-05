package day05

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSolution(t *testing.T) {
	Solution.VerifySolution(t, 5, 7142, 20012)
}

func TestExample(t *testing.T) {
	example := `0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`
	Solution.VerifyInput(t, example, 5, 12)
}

func Test_intsBetween(t *testing.T) {
	tests := []struct {
		v1, v2   int
		expected []int
	}{
		{0, 3, []int{0, 1, 2, 3}},
		{3, 0, []int{0, 1, 2, 3}},
		{-3, 3, []int{-3, -2, -1, 0, 1, 2, 3}},
		{3, 3, []int{3}},
	}
	for _, tt := range tests {
		t.Run(fmt.Sprintf("%d -> %d", tt.v1, tt.v2), func(t *testing.T) {
			output := intsBetween(tt.v1, tt.v2)
			assert.ElementsMatch(t, tt.expected, output)
		})
	}
}
