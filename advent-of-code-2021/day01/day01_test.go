package day01

import (
	"testing"
)

func TestSolution(t *testing.T) {
	Solution.VerifySolution(t, 1, 1184, 1158)
}

func TestExample(t *testing.T) {
	example := `199
200
208
210
200
207
240
269
260
263`
	Solution.VerifyInput(t, example, 7, 5)
}
