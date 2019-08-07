package firewall

import (
	"fmt"
	"strings"
)

type Firewall struct {
	maxDepth int
	layers   map[int]*layer
}

func Init(input string) Firewall {
	var layers []layer

	for _, line := range strings.Split(input, "\n") {
		var depth, scannerRange int

		_, err := fmt.Sscanf(line, "%d: %d", &depth, &scannerRange)
		if err != nil {
			panic(err)
		}

		layers = append(layers, initLayer(depth, scannerRange))
	}

	return initFromLayers(layers)
}

func initFromLayers(layers []layer) (f Firewall) {
	f.layers = make(map[int]*layer)

	for i := range layers {
		l := &layers[i]

		f.layers[l.depth] = l

		if l.depth > f.maxDepth {
			f.maxDepth = l.depth
		}
	}

	return
}

func (f *Firewall) SeverityAccruedAfterSteppingThrough() (severityAccrued int) {
	for pos := 0; pos <= f.maxDepth; pos += 1 {

		l, ok := f.layers[pos]
		if ok && l.isScannerAtZero() {
			severityAccrued += l.severity()
		}

		for _, l := range f.layers {
			l.step()
		}
	}

	for _, l := range f.layers {
		l.reset()
	}

	return severityAccrued
}

func (f *Firewall) CanStepThroughWithoutGettingCaughtAfter(steps int) bool {
	for _, l := range f.layers {
		if l.willScannerBeAtZeroIn(steps + l.depth) {
			return false
		}
	}
	return true
}
