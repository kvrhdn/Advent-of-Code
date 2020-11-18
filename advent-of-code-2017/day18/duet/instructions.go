package duet

import (
	"fmt"
	"github.com/pkg/errors"
	"strings"
)

type Instruction interface {
	operateOn(c *Controller) (shouldHalt bool)
}

func ParseInstructions(in string) (instructions []Instruction, err error) {
	for _, line := range strings.Split(in, "\n") {
		i, err := parseInstruction(line)
		if err != nil {
			return instructions, err
		}
		instructions = append(instructions, i)
	}
	return
}

func parseInstruction(in string) (ir Instruction, err error) {
	fields := strings.Fields(in)

	opcode := fields[0]
	args := fields[1:]

	switch opcode {
	case "snd":
		return newSnd(args)

	case "set":
		return newSet(args)

	case "add":
		return newAdd(args)

	case "mul":
		return newMul(args)

	case "mod":
		return newMod(args)

	case "rcv":
		return newRcv(args)

	case "jgz":
		return newJgz(args)

	default:
		err = errors.New(fmt.Sprintf("could not recognize instruction %q", opcode))
		return
	}
}

type set struct {
	reg register
	val value
}

func newSet(args []string) (s set, err error) {
	s.reg, s.val, err = parseAsRegAndVal(args)
	return
}

func (s set) operateOn(c *Controller) (shouldHalt bool) {
	c.set(s.reg, s.val.fetch(c))
	return
}

type add struct {
	reg register
	val value
}

func newAdd(args []string) (a add, err error) {
	a.reg, a.val, err = parseAsRegAndVal(args)
	return
}

func (a add) operateOn(c *Controller) (shouldHalt bool) {
	c.set(a.reg, c.get(a.reg)+a.val.fetch(c))
	return
}

type mul struct {
	reg register
	val value
}

func newMul(args []string) (m mul, err error) {
	m.reg, m.val, err = parseAsRegAndVal(args)
	return
}

func (m mul) operateOn(c *Controller) (shouldHalt bool) {
	c.set(m.reg, c.get(m.reg)*m.val.fetch(c))
	return
}

type mod struct {
	reg register
	val value
}

func newMod(args []string) (m mod, err error) {
	m.reg, m.val, err = parseAsRegAndVal(args)
	return
}

func (m mod) operateOn(c *Controller) (shouldHalt bool) {
	c.set(m.reg, c.get(m.reg)%m.val.fetch(c))
	return
}

type jgz struct {
	value  value
	offset value
}

func newJgz(args []string) (j jgz, err error) {
	j.value, j.offset, err = parseAsValAndVal(args)
	return
}

func (j jgz) operateOn(c *Controller) (shouldHalt bool) {
	if j.value.fetch(c) > 0 {
		c.jump(j.offset.fetch(c))
	}
	return
}

type snd struct {
	val value
}

func newSnd(args []string) (s snd, err error) {
	s.val, err = parseAsVal(args)
	return
}

func (s snd) operateOn(c *Controller) (shouldHalt bool) {
	switch c.version {
	case v1:
		c.playSound(s.val.fetch(c))

	case v2:
		shouldHalt = !c.send(s.val.fetch(c))

	}
	return
}

type rcv struct {
	reg register
}

func newRcv(args []string) (r rcv, err error) {
	r.reg, err = parseAsReg(args)
	return
}

func (r rcv) operateOn(c *Controller) (shouldHalt bool) {
	switch c.version {
	case v1:
		if c.get(r.reg) != 0 {
			shouldHalt = true
		}

	case v2:
		val, timedOut := c.recv()
		if timedOut {
			shouldHalt = true
			break
		}

		c.set(r.reg, val)

	}
	return
}
