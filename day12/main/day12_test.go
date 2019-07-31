package main

import (
	"github.com/koenaad/Advent-of-Code-2017/day12/program"
	"testing"
)

func Test_exampleInput(t *testing.T) {
	var input = `0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5`

	reports := program.ParseListOfReachabilityReports(input)
	groups := program.DivideIntoReachabilityGroups(reports)

	gotSizeOfGroupWithProgram0 := sizeOfGroupWithProgram0(groups)
	if gotSizeOfGroupWithProgram0 != 6 {
		t.Errorf("sizeOfGroupWithProgram0: got %v, but expected 6", gotSizeOfGroupWithProgram0)
	}

	gotAmountOfGroups := len(groups)
	if gotAmountOfGroups != 2 {
		t.Errorf("len(groups): got %v, but expected 2", gotAmountOfGroups)
	}
}
