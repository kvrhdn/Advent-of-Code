package firewall

import "testing"

func Test_layer_step(t *testing.T) {
	l := initLayer(0, 3)

	expectScannerPosition(t, l, 0)

	l.step()
	expectScannerPosition(t, l, 1)

	l.step()
	expectScannerPosition(t, l, 2)

	l.step()
	expectScannerPosition(t, l, 1)

	l.step()
	expectScannerPosition(t, l, 0)

	l.step()
	expectScannerPosition(t, l, 1)
}

func expectScannerPosition(t *testing.T, l layer, expectedPosition int) {
	if l.scannerPosition != expectedPosition {
		t.Errorf("Expected position scanner at %v, but was at %v", expectedPosition, l.scannerPosition)
	}
}
