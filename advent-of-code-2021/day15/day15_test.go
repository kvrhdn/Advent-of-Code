package day15

import (
	"testing"
)

func TestSolution(t *testing.T) {
	Solution.VerifySolution(t, 15, 537, 2881)
}

func TestExample(t *testing.T) {
	example := `1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581`
	Solution.VerifyInput(t, example, 40, 315)
}
