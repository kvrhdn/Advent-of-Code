package main

import (
	"testing"
)

func TestManhattanDistanceFromStorageLocationOf(t *testing.T) {
	cases := []struct {
		in       int
		expected int
	}{
		{1, 0},
		{12, 3},
		{23, 2},
		{1024, 31},
	}
	for _, c := range cases {
		got := DetermineDistanceToStorageLocationInSpiralMemory(c.in)
		if got != c.expected {
			t.Errorf("ManhattanDistanceFromStorageLocationOf(%v) = %v, but expected %v", c.in, got, c.expected)
		}
	}
}

func TestFirstLoadtestValueGreaterThan(t *testing.T) {
	cases := []struct {
		in       int
		expected int
	}{
		{2, 4},
		{10, 11},
		{55, 57},
		{150, 304},
		{750, 806},
	}
	for _, c := range cases {
		got := FirstLoadtestValueGreaterThan(c.in)
		if got != c.expected {
			t.Errorf("FirstLoadtestValueGreaterThan(%v) = %v, but expected %v", c.in, got, c.expected)
		}
	}
}
