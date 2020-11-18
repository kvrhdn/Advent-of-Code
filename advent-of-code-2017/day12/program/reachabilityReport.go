package program

import (
	"fmt"
	"strconv"
	"strings"
)

type ReachabilityReport struct {
	Program            ProgramId
	CanCommunicateWith []ProgramId
}

func parseReachabilityReport(line string) ReachabilityReport {
	var report ReachabilityReport

	_, err := fmt.Sscanf(line, "%d <-> ", &report.Program)
	if err != nil {
		panic(fmt.Sprintf("could not parse input %q", line))
	}

	indexOfDivider := strings.Index(line, "<-> ")
	if indexOfDivider > 0 {
		listOfIds := strings.Split(line[indexOfDivider+4:], ", ")
		for _, pString := range listOfIds {
			p, err := strconv.Atoi(pString)
			if err != nil {
				panic(fmt.Sprintf("could not convert \"%v\" to integer", pString))
			}

			report.CanCommunicateWith = append(report.CanCommunicateWith, ProgramId(p))
		}
	}

	return report
}

func ParseListOfReachabilityReports(input string) []ReachabilityReport {
	lines := strings.Split(input, "\n")

	reports := make([]ReachabilityReport, 0, len(lines))

	for _, line := range lines {
		reports = append(reports, parseReachabilityReport(line))
	}

	return reports
}
