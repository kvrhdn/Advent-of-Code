package program

import (
	"reflect"
	"testing"
)

func Test_parseReachabilityReport(t *testing.T) {
	cases := []struct {
		input          string
		expectedReport ReachabilityReport
	}{
		{"0 <-> 2", ReachabilityReport{0, []ProgramId{2}}},
		{"1 <-> 1", ReachabilityReport{1, []ProgramId{1}}},
		{"2 <-> 0, 3, 4", ReachabilityReport{2, []ProgramId{0, 3, 4}}},
		{"3 <-> 2, 4", ReachabilityReport{3, []ProgramId{2, 4}}},
		{"4 <-> 2, 3, 6", ReachabilityReport{4, []ProgramId{2, 3, 6}}},
		{"5 <-> 6", ReachabilityReport{5, []ProgramId{6}}},
		{"6 <-> 4, 5", ReachabilityReport{6, []ProgramId{4, 5}}},
	}
	for _, c := range cases {
		report := parseReachabilityReport(c.input)
		if !reflect.DeepEqual(c.expectedReport, report) {
			t.Errorf("parseReachabilityReport(%q): got %v, but expected %v", c.input, report, c.expectedReport)
		}
	}
}

func TestParseListOfReachabilityReports(t *testing.T) {
	var input = `0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5`

	var expectedReports = []ReachabilityReport{
		{0, []ProgramId{2}},
		{1, []ProgramId{1}},
		{2, []ProgramId{0, 3, 4}},
		{3, []ProgramId{2, 4}},
		{4, []ProgramId{2, 3, 6}},
		{5, []ProgramId{6}},
		{6, []ProgramId{4, 5}},
	}

	got := ParseListOfReachabilityReports(input)
	if !reflect.DeepEqual(expectedReports, got) {
		t.Errorf("ParseListOfReachabilityReports(%q): got %v, but expected %v", input, got, expectedReports)
	}
}
