package day06

import (
	"testing"
)

func TestSolution(t *testing.T) {
	Solution.VerifySolution(t, 6, 388419, 1740449478328)
}

func TestExample(t *testing.T) {
	example := "3,4,3,1,2"
	Solution.VerifyInput(t, example, 5934, 26984457539)
}
