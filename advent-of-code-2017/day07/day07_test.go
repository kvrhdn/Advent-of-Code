package day07

import (
	"reflect"
	"testing"
)

func TestParseProgram(t *testing.T) {
	var cases = []struct {
		in       string
		expected Program
	}{
		{"pbga (66)", Program{Name: "pbga", Weight: 66}},
		{"fwft (72) -> ktlj, cntj, xhth", Program{Name: "fwft", Weight: 72, HoldsNames: []string{"ktlj", "cntj", "xhth"}}},
	}
	for _, c := range cases {
		got := ParseProgram(c.in)
		if !reflect.DeepEqual(got, c.expected) {
			t.Errorf("ParseProgram(%q) = %v, but expected %v", c.in, got, c.expected)
		}
	}
}

func TestParseInput(t *testing.T) {
	programTower := loadExampleInput()

	if programTower.Name != "tknk" {
		t.Errorf("found bottom program %v, but expected \"tknk\"", programTower.Name)
	}
}

func TestProgram_RecursiveWeight(t *testing.T) {
	programTower := loadExampleInput()

	got := programTower.RecursiveWeight()
	if got != 778 {
		t.Errorf("expected recursive weight to be 778, but got %v", got)
	}
}

func TestFindUnbalancedProgram(t *testing.T) {
	programTower := loadExampleInput()

	got := FindTopMostUnbalancedProgram(programTower)

	if got.Name != "tknk" {
		t.Errorf("expected unbalanced program to be \"tknk\", but got %v", got.Name)
	}
}

func loadExampleInput() Program {
	input := `pbga (66)
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
	programTower := ParseInput(input)
	return programTower
}
