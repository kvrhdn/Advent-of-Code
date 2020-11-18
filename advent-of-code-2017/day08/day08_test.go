package day08

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day08/instruction"
)

func TestEvaluateOn(t *testing.T) {
	instructions := loadExampleInput()

	bank := instruction.NewRegisterBank()
	bank.EvaluateAll(instructions)

	for k, v := range bank {
		switch k {
		case "a":
			if v != 1 {
				t.Errorf("expected register \"a\" to be 1, but got %v", v)
			}
		case "b":
			if v != 0 {
				t.Errorf("expected register \"b\" to be 0, but got %v", v)
			}
		case "c":
			if v != -10 {
				t.Errorf("expected register \"c\" to be -10, but got %v", v)
			}
		default:
			t.Errorf("did not expected register %q to contain a value %v", k, v)
		}
	}
}

func Test_registerWithLargestValue(t *testing.T) {
	instructions := loadExampleInput()

	bank := instruction.NewRegisterBank()
	bank.EvaluateAll(instructions)

	gotRegister, gotValue := registerWithLargestValue(bank)

	if gotRegister != "a" || gotValue != 1 {
		t.Errorf("expected register \"a\" to be largest (1), but got %q (%v)", gotRegister, gotValue)
	}
}

func Test_registerWithLargestValueDuringEvaluation(t *testing.T) {
	instructions := loadExampleInput()

	bank := instruction.NewRegisterBank()

	gotRegister, gotValue := registerWithLargestValueDuringEvaluation(instructions, bank)

	if gotRegister != "c" || gotValue != 10 {
		t.Errorf("expected register \"c\" to be largest (10), but got %q (%v)", gotRegister, gotValue)
	}
}

func loadExampleInput() []instruction.JumpInstruction {
	input := `b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10`

	instructions := instruction.ParseListOfJumpInstructions(input)
	return instructions
}
