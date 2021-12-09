package day09

import (
	"testing"
)

func TestSolution(t *testing.T) {
	Solution.VerifySolution(t, 9, 545, 950600)
}

func TestExample(t *testing.T) {
	example := `2199943210
3987894921
9856789892
8767896789
9899965678`
	Solution.VerifyInput(t, example, 15, 1134)
}
