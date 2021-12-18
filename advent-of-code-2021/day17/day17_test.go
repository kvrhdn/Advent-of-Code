package day17

import (
	"testing"
)

func TestSolution(t *testing.T) {
	Solution.VerifySolution(t, 17, 2850, 1117)
}

func TestExample(t *testing.T) {
	example := `target area: x=20..30, y=-10..-5`
	Solution.VerifyInput(t, example, 45, 112)
}
