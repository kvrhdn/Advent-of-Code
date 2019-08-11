package dance

import "testing"

func TestDance(t *testing.T) {
	inputDancers := "abcde"
	inputMoves := []Move{spin{1}, exchange{3, 4}, partner{'e', 'b'}}

	expected := "baedc"

	got := Dance(inputDancers, inputMoves)
	if got != expected {
		t.Errorf("Dance(...) = %v, but expected %v", got, expected)
	}
}
