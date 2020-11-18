package firewall

import (
	"reflect"
	"testing"
)

var exampleInput = `0: 3
1: 2
4: 4
6: 4`

var exampleLayers = []layer{
	{0, 3, 0, 0},
	{1, 2, 0, 0},
	{4, 4, 0, 0},
	{6, 4, 0, 0},
}

func TestInit(t *testing.T) {
	f := Init(exampleInput)

	if f.maxDepth != 6 {
		t.Errorf("Expected max layer to be 6, but got %v", f.maxDepth)
	}

	for _, expectedLayer := range exampleLayers {
		l, ok := f.layers[expectedLayer.depth]

		if !ok || reflect.DeepEqual(l, expectedLayer) {
			t.Errorf("Expected layer %v, but got %v", expectedLayer, l)
		}
	}
}

func TestFirewall_MoveThrough(t *testing.T) {
	f := initFromLayers(exampleLayers)

	expectedSeverity := 24

	gotSeverity := f.SeverityAccruedAfterSteppingThrough()

	if gotSeverity != expectedSeverity {
		t.Errorf("Moved through firewall expected severity %v, but got %v", expectedSeverity, gotSeverity)
	}
}
