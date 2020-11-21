package day07

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

const exampleInput = `pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)`

func TestParseInput(t *testing.T) {
	programs := parseInput(exampleInput)

	assert.Equal(t,
		ProgramDescription{
			Name:      "pbga",
			Weight:    66,
			IsHolding: nil,
		},
		programs["pbga"],
	)
	assert.Equal(t,
		ProgramDescription{
			Name:      "fwft",
			Weight:    72,
			IsHolding: []string{"ktlj", "cntj", "xhth"},
		},
		programs["fwft"],
	)
}

func TestFindBottomProgram(t *testing.T) {
	programs := parseInput(exampleInput)

	assert.Equal(t, "tknk", findBottomProgram(programs).Name)
}

func TestExamplePart2(t *testing.T) {
	programs := parseInput(exampleInput)
	bottom := findBottomProgram(programs)

	programTree := buildProgramTree(&bottom, programs)

	unbalancedProgram, expectedTotalWeight := programTree.findUnbalancedProgram()

	assert.Equal(t, "ugml", unbalancedProgram.Name)
	assert.Equal(t, 243, expectedTotalWeight)
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 7, "../")

	assert.Equal(t, "dtacyn", SolvePart1(input))
	assert.Equal(t, 521, SolvePart2(input))
}
