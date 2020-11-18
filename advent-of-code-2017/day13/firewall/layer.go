package firewall

type layer struct {
	depth            int
	scannerRange     int
	scannerPosition  int
	scannerDirection int
}

func initLayer(depth, scannerRange int) layer {
	return layer{
		depth:        depth,
		scannerRange: scannerRange,
	}
}

func (l *layer) step() {
	if l.scannerPosition == 0 {
		l.scannerDirection = 1
	}
	if l.scannerPosition == l.scannerRange-1 {
		l.scannerDirection = -1
	}

	l.scannerPosition += l.scannerDirection
}

func (l *layer) isScannerAtZero() bool {
	return l.scannerPosition == 0
}

func (l *layer) severity() int {
	return l.depth * l.scannerRange
}

func (l *layer) reset() {
	l.scannerPosition = 0
}

func (l *layer) willScannerBeAtZeroIn(steps int) bool {
	// unwrapped range is the amount of steps to return to the zero position
	//
	// i.e. if scanner range = 3
	//	0 1 2 1 0 --> unwrapped range = 4
	//  | - - |
	//
	// if scanner range = 5
	//	0 1 2 3 4 3 2 1 0 --> unwrapped range = 8
	//	| - - - - - - |
	//
	unwrappedRange := (2 * l.scannerRange) - 2

	unwrappedPosition := steps % unwrappedRange

	return unwrappedPosition == 0
}
