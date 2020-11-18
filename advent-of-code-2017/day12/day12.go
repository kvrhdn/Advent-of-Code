package day12

import (
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day12/program"
)

func SolvePart1(input string) interface{} {
	reports := program.ParseListOfReachabilityReports(input)
	groups := program.DivideIntoReachabilityGroups(reports)

	return sizeOfGroupWithProgram0(groups)
}

func SolvePart2(input string) interface{} {
	reports := program.ParseListOfReachabilityReports(input)
	groups := program.DivideIntoReachabilityGroups(reports)

	return len(groups)
}

func sizeOfGroupWithProgram0(groups []program.ProgramSet) int {
	for _, group := range groups {
		if group.Contains(program.ProgramId(0)) {
			return len(group)
		}
	}
	panic("could not find group with program 0")
}
