package day13

import (
	"fmt"
)

type Layer struct {
	depth        int
	scannerRange int
}

func parseLayer(input string) (l Layer) {
	_, err := fmt.Sscanf(input, "%d: %d", &l.depth, &l.scannerRange)
	if err != nil {
		panic(err)
	}
	return
}

func (l *Layer) isScannerAtZeroAfter(steps int) bool {
	// unwrappedRange is the amount of steps to return to the zero position, i.e.
	// if scanner range = 3
	//   0 1 2 1 0 --> unwrapped range = 4
	//   | - - |
	//
	// if scanner range = 5
	//   0 1 2 3 4 3 2 1 0 --> unwrapped range = 8
	//   | - - - - - - |
	unwrappedRange := (2 * l.scannerRange) - 2

	return steps%unwrappedRange == 0
}
