package day08

import (
	"fmt"
)

type Operation func(memory map[string]int)
type Condition func(memory map[string]int) bool

type Instruction struct {
	op   Operation
	cond Condition
}

func parseInstruction(input string) Instruction {
	var opReg, opOp, condReg, condOp string
	var opArg, condArg int

	_, err := fmt.Sscanf(
		input,
		"%s %s %d if %s %s %d",
		&opReg, &opOp, &opArg, &condReg, &condOp, &condArg,
	)
	if err != nil {
		panic(err)
	}

	return Instruction{
		op:   makeOperation(opReg, opOp, opArg),
		cond: makeCondition(condReg, condOp, condArg),
	}
}

func makeOperation(reg string, op string, arg int) Operation {
	switch op {
	case "inc":
		return func(memory map[string]int) {
			memory[reg] += arg
		}
	case "dec":
		return func(memory map[string]int) {
			memory[reg] -= arg
		}
	default:
		panic(fmt.Sprintf("could not create Operation, unknown op = %v", op))
	}
}

func makeCondition(reg string, op string, arg int) Condition {
	switch op {
	case "==":
		return func(memory map[string]int) bool {
			return memory[reg] == arg
		}
	case "!=":
		return func(memory map[string]int) bool {
			return memory[reg] != arg
		}
	case ">":
		return func(memory map[string]int) bool {
			return memory[reg] > arg
		}
	case "<":
		return func(memory map[string]int) bool {
			return memory[reg] < arg
		}
	case ">=":
		return func(memory map[string]int) bool {
			return memory[reg] >= arg
		}
	case "<=":
		return func(memory map[string]int) bool {
			return memory[reg] <= arg
		}
	default:
		panic(fmt.Sprintf("could not create Condition, unknown op = %v", op))
	}
}

func (ir *Instruction) execute(memory map[string]int) {
	if ir.cond(memory) {
		ir.op(memory)
	}
}
