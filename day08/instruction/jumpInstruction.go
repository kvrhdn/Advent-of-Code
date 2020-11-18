package instruction

import (
	"fmt"
	"strings"
)

type operationFunc func(bank RegisterBank)
type conditionFunc func(bank RegisterBank) bool

type JumpInstruction struct {
	operation operationFunc
	condition conditionFunc
}

func (ir *JumpInstruction) evaluateOn(bank RegisterBank) {
	if ir.condition(bank) {
		ir.operation(bank)
	}
}

func ParseListOfJumpInstructions(input string) []JumpInstruction {
	lines := strings.Split(input, "\n")

	irs := make([]JumpInstruction, len(lines))

	for i, line := range lines {
		irs[i] = parseJumpInstruction(line)
	}

	return irs
}

func parseJumpInstruction(input string) JumpInstruction {
	var opRegister string
	var opOperator string
	var opArg int

	var condRegister string
	var condOperator string
	var condArg int

	n, err := fmt.Sscanf(input, "%s %s %d if %s %s %d", &opRegister, &opOperator, &opArg, &condRegister, &condOperator, &condArg)
	if err != nil {
		panic(fmt.Sprintf("could not correctly parse JumpInstruction: n = %v, err = %v", n, err))
	}

	op := makeOperation(opRegister, opOperator, opArg)
	cond := makeCondition(condRegister, condOperator, condArg)

	return JumpInstruction{op, cond}
}

func makeOperation(opRegister string, opOperator string, opArg int) operationFunc {
	switch opOperator {
	case "inc":
		return func(bank RegisterBank) {
			bank[opRegister] += opArg
		}

	case "dec":
		return func(bank RegisterBank) {
			bank[opRegister] -= opArg
		}

	default:
		panic(fmt.Sprintf("could not create operationFunc, unknown opOperator = %v", opOperator))
	}
}

func makeCondition(condRegister string, condOperator string, condArg int) conditionFunc {
	switch condOperator {
	case "==":
		return func(bank RegisterBank) bool {
			return bank[condRegister] == condArg
		}

	case "!=":
		return func(bank RegisterBank) bool {
			return bank[condRegister] != condArg
		}

	case ">":
		return func(bank RegisterBank) bool {
			return bank[condRegister] > condArg
		}

	case "<":
		return func(bank RegisterBank) bool {
			return bank[condRegister] < condArg
		}

	case ">=":
		return func(bank RegisterBank) bool {
			return bank[condRegister] >= condArg
		}

	case "<=":
		return func(bank RegisterBank) bool {
			return bank[condRegister] <= condArg
		}

	default:
		panic(fmt.Sprintf("could not create conditionFunc, unknown condOperator = %v", condOperator))
	}
}
