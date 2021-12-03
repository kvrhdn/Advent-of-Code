package day03

import (
	"testing"
)

func TestSolution(t *testing.T) {
	Solution.VerifySolution(t, 3, 3813416, 2990784)
}

func TestExample(t *testing.T) {
	example := `00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010`
	Solution.VerifyInput(t, example, 198, 230)
}
