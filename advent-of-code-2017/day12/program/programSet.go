package program

import "reflect"

type ProgramSet map[ProgramId]bool

func newProgramSet() ProgramSet {
	return make(map[ProgramId]bool)
}

func (set *ProgramSet) Contains(p ProgramId) bool {
	for k, v := range *set {
		if k == p && v == true {
			return true
		}
	}
	return false
}

func (set *ProgramSet) containsAny(ps []ProgramId) bool {
	for _, p := range ps {
		if set.Contains(p) {
			return true
		}
	}
	return false
}

func (set *ProgramSet) add(ProgramIds ...ProgramId) {
	for _, p := range ProgramIds {
		(*set)[p] = true
	}
}

func (set *ProgramSet) addSet(otherSet ProgramSet) {
	for k, v := range otherSet {
		(*set)[k] = v
	}
}

func DivideIntoReachabilityGroups(reports []ReachabilityReport) []ProgramSet {
	var programSets []ProgramSet

	for _, report := range reports {
		programSets = applyReachabilityReport(programSets, report)
	}

	return programSets
}

func applyReachabilityReport(programSets []ProgramSet, report ReachabilityReport) []ProgramSet {
	programs := []ProgramId{report.Program}
	programs = append(programs, report.CanCommunicateWith...)

	for _, set1 := range programSets {
		// an ProgramId is already present in programSets (as part of an earlier link)
		if set1.containsAny(programs) {
			// add all ProgramId
			set1.add(programs...)

			// for each ProgramId, check if present in other sets --> merge into set1...
			for _, p := range programs {
				for i, set2 := range programSets {

					if reflect.DeepEqual(set1, set2) {
						continue
					}

					if set2.Contains(p) {
						set1.addSet(set2)

						// ...and remove set2 from programSets
						programSets = append(programSets[:i], programSets[i+1:]...)
					}
				}
			}
			return programSets
		}
	}

	// all ProgramId are not yet in programSets, create a new set
	newSet := newProgramSet()
	newSet.add(programs...)

	programSets = append(programSets, newSet)

	return programSets
}
