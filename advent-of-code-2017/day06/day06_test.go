package day06

import "testing"

func TestRedistributeCyclesUntilLoop(t *testing.T) {
	input := []int{0, 2, 7, 0}
	expectedCycles := 5
	expectedLoopSize := 4

	gotCycles, gotLoopSize := RedistributeCyclesUntilLoop(input)

	if gotCycles != expectedCycles || gotLoopSize != expectedLoopSize {
		t.Errorf("RedistributeCyclesUntilLoop(%v): got %v, %v but expected %v, %v", input, gotCycles, gotLoopSize, expectedCycles, expectedLoopSize)
	}
}
