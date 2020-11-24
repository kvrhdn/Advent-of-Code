package day23

import (
	"strconv"
	"strings"
)

type InstructionType string

const (
	Set InstructionType = "set"
	Sub InstructionType = "sub"
	Mul InstructionType = "mul"
	Jnz InstructionType = "jnz"
)

type Instruction struct {
	t    InstructionType
	arg0 string
	arg1 string
}

func parseInstruction(input string) (ir Instruction) {
	fields := strings.Fields(input)

	return Instruction{
		t:    InstructionType(fields[0]),
		arg0: fields[1],
		arg1: fields[2],
	}
}

type Coprocessor struct {
	reg map[rune]int
	ir  int

	mulCalled int
}

func newProcessor() Coprocessor {
	return Coprocessor{
		reg: map[rune]int{
			'a': 0,
			'b': 0,
			'c': 0,
			'd': 0,
			'e': 0,
			'f': 0,
			'g': 0,
			'h': 0,
		},
		ir:        0,
		mulCalled: 0,
	}
}

func (c *Coprocessor) execute(instructions []Instruction) {
	for {
		if c.ir < 0 || c.ir >= len(instructions) {
			return
		}

		ir := instructions[c.ir]

		switch ir.t {
		case Set:
			c.modify(ir.arg0, func(value int) int {
				return c.resolve(ir.arg1)
			})
		case Sub:
			c.modify(ir.arg0, func(value int) int {
				return value - c.resolve(ir.arg1)
			})
		case Mul:
			c.modify(ir.arg0, func(value int) int {
				return value * c.resolve(ir.arg1)
			})
			c.mulCalled++
		case Jnz:
			if c.resolve(ir.arg0) != 0 {
				c.ir += c.resolve(ir.arg1) - 1
			}
		}

		c.ir++
	}
}

func (c *Coprocessor) modify(reg string, f func(int) int) {
	regRune := []rune(reg)[0]
	c.reg[regRune] = f(c.reg[regRune])
}

func (c *Coprocessor) resolve(arg string) int {
	value, err := strconv.Atoi(arg)
	if err == nil {
		return value
	}
	regRune := []rune(arg)[0]
	return c.reg[regRune]
}
