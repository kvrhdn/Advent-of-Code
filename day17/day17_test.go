package main

import "testing"

func Test_spinlockAlgorithm(t *testing.T) {
	steps := 3
	expected := 638

	got := spinlockAlgorithmFindValueAfter2017(steps)
	if got != expected {
		t.Errorf("spinlockAlgorithmFindValueAfter2017(%v) = %v, but expected %v", steps, got, expected)
	}
}

func Test_spinlockAlgorithmFindValueAfter0(t *testing.T) {
	steps := 3
	expected := 5
	stopAt := 5

	// 0
	// 0 1
	// 0 2 1
	// 0 2 3 1
	// 0 2 4 3 1
	// 0 5 2 4 3 1

	got := spinlockAlgorithmFindValueAfter0(steps, stopAt)
	if got != expected {
		t.Errorf("spinlockAlgorithmFindValueAfter0(%v, %v) = %v, but expected %v", steps, stopAt, got, expected)
	}
}
