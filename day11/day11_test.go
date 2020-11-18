package main

import (
	"testing"
)

func TestPositionAfterSteps(t *testing.T) {
	cases := []struct {
		steps            []string
		expectedDistance float64
	}{
		{[]string{"ne", "ne", "ne"}, 3},
		{[]string{"ne", "ne", "sw", "sw"}, 0},
		{[]string{"ne", "ne", "s", "s"}, 2},
		{[]string{"se", "sw", "se", "sw", "sw"}, 3},
	}
	for _, c := range cases {
		pos := Pos{}
		pos.TakeSteps(c.steps)

		gotDistance := pos.DistanceFromOrigin()

		if c.expectedDistance != gotDistance {
			t.Errorf("distance after TakeSteps(%v): got %v, but expected %v", c.steps, gotDistance, c.expectedDistance)
		}
	}
}
