package program

import (
	"reflect"
	"testing"
)

func Test_ProgramSet(t *testing.T) {
	testProgramId := ProgramId(5)

	programSet := newProgramSet()

	if programSet.Contains(testProgramId) {
		t.Errorf("program set already contains test program id, programSet = %v", programSet)
	}

	programSet.add(testProgramId)

	if !programSet.Contains(testProgramId) {
		t.Errorf("program set does not contain test program id, programSet = %v", programSet)
	}

	programSet2 := newProgramSet()
	programSet2.addSet(programSet)

	if !programSet2.Contains(testProgramId) {
		t.Errorf("program set 2 does not contain test program id, programSet = %v, programSet2 = %v", programSet, programSet2)
	}
}

func Test_applyReachabilityReport(t *testing.T) {
	var programSets, expectedProgramSets []ProgramSet

	programSets = applyReachabilityReport(programSets, ReachabilityReport{0, []ProgramId{2}})

	expectedProgramSets = []ProgramSet{{0: true, 2: true}}

	if !reflect.DeepEqual(programSets, expectedProgramSets) {
		t.Errorf("applyReachabilityReport 1, expected %v but got %v", expectedProgramSets, programSets)
	}

	programSets = applyReachabilityReport(programSets, ReachabilityReport{1, []ProgramId{3}})

	expectedProgramSets = []ProgramSet{{0: true, 2: true}, {1: true, 3: true}}

	if !reflect.DeepEqual(programSets, expectedProgramSets) {
		t.Errorf("applyReachabilityReport 2, expected %v but got %v", expectedProgramSets, programSets)
	}

	programSets = applyReachabilityReport(programSets, ReachabilityReport{5, []ProgramId{2, 1}})

	expectedProgramSets = []ProgramSet{{0: true, 2: true, 1: true, 3: true, 5: true}}

	if !reflect.DeepEqual(programSets, expectedProgramSets) {
		t.Errorf("applyReachabilityReport 3, expected %v but got %v", expectedProgramSets, programSets)
	}
}
