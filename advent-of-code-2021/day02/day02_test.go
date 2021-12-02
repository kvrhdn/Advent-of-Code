package day02

import (
	"testing"
)

func TestSolution(t *testing.T) {
	Solution.VerifySolution(t, 2, 2039912, 1942068080)
}

func TestExample(t *testing.T) {
	example := `forward 5
down 5
forward 8
up 3
down 8
forward 2`
	Solution.VerifyInput(t, example, 150, 900)
}
