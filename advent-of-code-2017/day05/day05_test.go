package main

import "testing"

func TestJumpThroughInstructionsUntilOutOfBounds(t *testing.T) {
	input := []int{0, 3, 0, 1, -3}
	expected := 5

	got := JumpThroughInstructionsUntilOutOfBounds(input)

	if got != expected {
		t.Errorf("JumpThroughInstructionsUntilOutOfBounds(%v): got %v but expected %v", input, got, expected)
	}
}

func TestSpecialJumpThroughInstructionsUntilOutOfBounds(t *testing.T) {
	input := []int{0, 3, 0, 1, -3}
	expected := 10

	got := SpecialJumpThroughInstructionsUntilOutOfBounds(input)

	if got != expected {
		t.Errorf("SpecialJumpThroughInstructionsUntilOutOfBounds(%v): got %v but expected %v", input, got, expected)
	}
}
