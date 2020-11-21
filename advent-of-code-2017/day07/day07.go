package day07

import (
	"strings"
)

func SolvePart1(input string) interface{} {
	programs := parseInput(input)

	return findBottomProgram(programs).Name
}

func SolvePart2(input string) interface{} {
	programs := parseInput(input)

	bottomProgram := findBottomProgram(programs)
	programTree := buildProgramTree(&bottomProgram, programs)

	unbalanced, expectedTotalWeight := programTree.findUnbalancedProgram()

	diff := unbalanced.totalWeight() - expectedTotalWeight

	return unbalanced.Weight - diff
}

func parseInput(input string) map[string]ProgramDescription {
	programs := make(map[string]ProgramDescription)

	for _, line := range strings.Split(input, "\n") {
		p := parseProgram(line)
		programs[p.Name] = p
	}

	return programs
}

func findBottomProgram(programs map[string]ProgramDescription) ProgramDescription {
	heldPrograms := make(map[string]bool)

	// collect a set of the held programs
	for _, p := range programs {
		for _, heldProgram := range p.IsHolding {
			heldPrograms[heldProgram] = true
		}
	}

	// find the program that is not held
	for _, p := range programs {
		if _, ok := heldPrograms[p.Name]; !ok {
			return p
		}
	}

	panic("no bottom program could be found")
}

func buildProgramTree(base *ProgramDescription, programs map[string]ProgramDescription) ProgramTree {
	var holds []ProgramTree

	for _, programName := range base.IsHolding {
		program := programs[programName]
		holds = append(holds, buildProgramTree(&program, programs))
	}

	return ProgramTree{
		Name:   base.Name,
		Weight: base.Weight,
		Holds:  holds,
	}
}
