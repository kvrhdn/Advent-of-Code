package day25

import (
	"testing"
)

func TestSolution(t *testing.T) {
	Solution.VerifySolution(t, 25, 367, 0)
}

func TestExample(t *testing.T) {
	example := `v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>`
	Solution.VerifyInput(t, example, 58, 0)
}
