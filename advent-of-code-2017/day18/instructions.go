package day18

import (
	"fmt"
	"strings"
)

type Instruction interface {
	apply(c *Controller) (shouldHalt bool)
}

func parseInstruction(in string) Instruction {
	fields := strings.Fields(in)

	opcode, args := fields[0], fields[1:]

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
		panic(fmt.Sprintf("could not recognize instruction %q", opcode))
	}
}

type Set struct {
	reg Register
	val Value
}

func newSet(args []string) (s Set) {
	s.reg = parseRegister(args[0])
	s.val = parseValue(args[1])
	return
}

func (s Set) apply(c *Controller) (shouldHalt bool) {
	c.set(s.reg, s.val.resolve(c))
	return false
}

type add struct {
	reg Register
	val Value
}

func newAdd(args []string) (a add) {
	a.reg = parseRegister(args[0])
	a.val = parseValue(args[1])
	return
}

func (a add) apply(c *Controller) (shouldHalt bool) {
	c.set(a.reg, c.get(a.reg)+a.val.resolve(c))
	return false
}

type mul struct {
	reg Register
	val Value
}

func newMul(args []string) (m mul) {
	m.reg = parseRegister(args[0])
	m.val = parseValue(args[1])
	return
}

func (m mul) apply(c *Controller) (shouldHalt bool) {
	c.set(m.reg, c.get(m.reg)*m.val.resolve(c))
	return false
}

type mod struct {
	reg Register
	val Value
}

func newMod(args []string) (m mod) {
	m.reg = parseRegister(args[0])
	m.val = parseValue(args[1])
	return
}

func (m mod) apply(c *Controller) (shouldHalt bool) {
	c.set(m.reg, c.get(m.reg)%m.val.resolve(c))
	return false
}

type jgz struct {
	value  Value
	offset Value
}

func newJgz(args []string) (j jgz) {
	j.value = parseValue(args[0])
	j.offset = parseValue(args[1])
	return
}

func (j jgz) apply(c *Controller) (shouldHalt bool) {
	if j.value.resolve(c) > 0 {
		c.jump(j.offset.resolve(c))
	}
	return false
}

type snd struct {
	val Value
}

func newSnd(args []string) (s snd) {
	s.val = parseValue(args[0])
	return
}

func (s snd) apply(c *Controller) (shouldHalt bool) {
	switch c.version {
	case v1:
		c.playSound(s.val.resolve(c))
		return false

	case v2:
		return !c.send(s.val.resolve(c))

	default:
		panic("unknown version")
	}
}

type rcv struct {
	reg Register
}

func newRcv(args []string) (r rcv) {
	r.reg = parseRegister(args[0])
	return
}

func (r rcv) apply(c *Controller) (shouldHalt bool) {
	switch c.version {
	case v1:
		return c.get(r.reg) != 0

	case v2:
		val, timedOut := c.recv()
		if timedOut {
			return true
		}
		c.set(r.reg, val)
		return false

	default:
		panic("unknown version")
	}
}
