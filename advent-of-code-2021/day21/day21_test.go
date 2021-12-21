package day21

import (
	"testing"
)

func TestSolution(t *testing.T) {
	Solution.VerifySolution(t, 21, 1006866, 273042027784929)
}

func TestExample(t *testing.T) {
	example := `Player 1 starting position: 4
Player 2 starting position: 8`
	Solution.VerifyInput(t, example, 739785, 444356092776315)
}
