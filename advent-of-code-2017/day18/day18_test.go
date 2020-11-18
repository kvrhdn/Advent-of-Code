package day18

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day18/duet"
)

func Test_findLastPlayedFrequency(t *testing.T) {
	input := `set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2`
	instructions, err := duet.ParseInstructions(input)
	if err != nil {
		t.Fatal(err)
	}

	expected := 4

	got := findLastPlayedFrequency(instructions)
	if got != expected {
		t.Errorf("findLastPlayedFrequency(...) = %v, but expected %v", got, expected)
	}
}

func Test_valuesSentByProgram1(t *testing.T) {
	input := `snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d`
	instructions, err := duet.ParseInstructions(input)
	if err != nil {
		t.Fatal(err)
	}

	expected := 3

	got := valuesSentByProgram1(instructions)
	if got != expected {
		t.Errorf("valuesSentByProgram1(...) = %v, but expected %v", got, expected)
	}
}
