package day23

import (
	"strings"
)

func SolvePart1(input string) interface{} {
	_, mulCalled := runDirectAssembly(0)
	return mulCalled
}

func SolvePart2(input string) interface{} {
	return optimizedPart2()
}

func parseInput(input string) (irs []Instruction) {
	for _, line := range strings.Split(input, "\n") {
		irs = append(irs, parseInstruction(line))
	}
	return
}
