package day07

import (
	"fmt"
	"strings"
)

type ProgramDescription struct {
	Name      string
	Weight    int
	IsHolding []string
}

func parseProgram(line string) (p ProgramDescription) {
	_, err := fmt.Sscanf(line, "%s (%d)", &p.Name, &p.Weight)
	if err != nil {
		panic(err)
	}

	indexArrow := strings.Index(line, "-> ")
	if indexArrow != -1 {
		p.IsHolding = strings.Split(line[indexArrow+3:], ", ")
	}

	return p
}

type ProgramTree struct {
	Name   string
	Weight int
	Holds  []ProgramTree
}

func (p *ProgramTree) totalWeight() int {
	weight := p.Weight

	for _, programHeld := range p.Holds {
		weight += programHeld.totalWeight()
	}

	return weight
}

// isBalanced checks whether the ProgramTree is balanced. A ProgramTree is
// balanced if all held have the same total weight.
func (p *ProgramTree) isBalanced() bool {
	if len(p.Holds) <= 1 {
		return true
	}

	expectedWeight := p.Holds[0].totalWeight()

	for _, p := range p.Holds[1:] {
		if expectedWeight != p.totalWeight() {
			return false
		}
	}
	return true
}

// findUnbalancedProgram will iterate over the ProgramTree until it finds the
// last program that is not balanced. It then searches for the held programTree
// that has a different weight from the others.
func (p *ProgramTree) findUnbalancedProgram() (unbalanced *ProgramTree, expectedTotalWeight int) {
	// assumption: there will always be at most one program that is unbalanced
	for _, program := range p.Holds {
		if !program.isBalanced() {
			return program.findUnbalancedProgram()
		}
	}

	// all programTrees above are balanced, p is the last unbalanced program

	// find the total weight (and the corresponding program) that only occurs once
	weightCounts := make(map[int]int)
	weightPrograms := make(map[int]*ProgramTree)

	for i := 0; i < len(p.Holds); i++ {
		held := p.Holds[i]

		totalWeight := held.totalWeight()

		weightCounts[totalWeight] += 1
		// this overwrites the previous program if their total weights collide, but that's fine
		weightPrograms[totalWeight] = &held
	}

	for weight, count := range weightCounts {
		if count > 1 {
			expectedTotalWeight = weight
		}
		if count == 1 {
			unbalanced = weightPrograms[weight]
		}
	}

	return
}
