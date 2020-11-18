package util

import "testing"

func TestManhattenDistance(t *testing.T) {
	cases := []struct {
		in1, in2 Pos
		expected int
	}{
		{Pos{1, 2}, Pos{3, 4}, 4},
		{Pos{-3, 0}, Pos{1, 4}, 8},
	}
	for _, c := range cases {
		got := ManhattenDistance(c.in1, c.in2)
		if got != c.expected {
			t.Errorf("ManhattenDistance(%v, %v) = %v, but expected %v", c.in1, c.in2, got, c.expected)
		}
	}
}

func TestAbs(t *testing.T) {
	cases := []struct {
		in, expected int
	}{
		{4, 4},
		{-4, 4},
		{0, 0},
	}
	for _, c := range cases {
		got := Abs(c.in)
		if got != c.expected {
			t.Errorf("Abs(%v) = %v, but expected %v", c.in, got, c.expected)
		}
	}
}
