package grid

import (
	"testing"
)

func TestStep(t *testing.T) {
	cases := []struct {
		in       Pos
		dir      Dir
		expected Pos
	}{
		{Pos{0, 0}, North, Pos{0, -1}},
		{Pos{0, 0}, East, Pos{1, 0}},
		{Pos{0, 0}, South, Pos{0, 1}},
		{Pos{0, 0}, West, Pos{-1, 0}},
	}

	for _, c := range cases {
		got := Step(c.in, c.dir)

		if got != c.expected {
			t.Errorf("Step(%v, %v) = %v, expected %v", c.in, c.dir, got, c.expected)
		}
	}
}
