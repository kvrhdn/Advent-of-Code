package day08

import (
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day08/instruction"
)

func SolvePart1(input string) interface{} {
	instructions := instruction.ParseListOfJumpInstructions(input)

	bank := instruction.NewRegisterBank()
	bank.EvaluateAll(instructions)

	_, value := registerWithLargestValue(bank)
	return value
}

func SolvePart2(input string) interface{} {
	instructions := instruction.ParseListOfJumpInstructions(input)

	bank := instruction.NewRegisterBank()

	_, value := registerWithLargestValueDuringEvaluation(instructions, bank)
	return value
}

func registerWithLargestValue(bank instruction.RegisterBank) (name string, value int) {
	for reg, v := range bank {
		if v > value {
			name = reg
			value = v
		}
	}
	return
}

func registerWithLargestValueDuringEvaluation(instructions []instruction.JumpInstruction, bank instruction.RegisterBank) (name string, value int) {
	for _, ir := range instructions {
		bank.Evaluate(ir)

		largestRegister, largestValue := registerWithLargestValue(bank)
		if largestValue > value {
			name = largestRegister
			value = largestValue
		}
	}
	return
}
