package day07

import (
	"testing"
)

func TestSolution(t *testing.T) {
	Solution.VerifySolution(t, 7, 356992, 101268110)
}

func TestExample(t *testing.T) {
	example := "16,1,2,0,4,2,7,1,2,14"
	Solution.VerifyInput(t, example, 37, 168)
}
