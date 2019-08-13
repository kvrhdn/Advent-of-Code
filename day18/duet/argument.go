package duet

import (
	"fmt"
	"github.com/pkg/errors"
	"strconv"
)

func asReg(s string) (r register, err error) {
	if len(s) != 1 {
		return r, errors.New(fmt.Sprintf("could not parse argument %q as register: length is not 1", s))
	}
	return register([]rune(s)[0]), nil
}

// value can either be a constant value or a reference to a value stored in a register to be resolved at run-time
type value interface {
	fetch(c *Controller) int
}

func asValue(s string) (v value, err error) {
	i, err := strconv.Atoi(s)
	if err == nil {
		return valueInt(i), nil
	}
	r, err := asReg(s)
	if err == nil {
		return valueReg(r), nil
	}
	return v, errors.New(fmt.Sprintf("could not process %q as int or register", s))
}

type valueInt int

func (v valueInt) fetch(c *Controller) int {
	return int(v)
}

type valueReg register

func (v valueReg) fetch(c *Controller) int {
	return c.get(register(v))
}

func parseAsReg(args []string) (reg register, err error) {
	if len(args) != 1 {
		err = errors.New("expecting 1 argument")
		return
	}
	return asReg(args[0])
}

func parseAsVal(args []string) (val value, err error) {
	if len(args) != 1 {
		err = errors.New("expecting 1 argument")
		return
	}
	return asValue(args[0])
}

func parseAsRegAndVal(args []string) (reg register, val value, err error) {
	if len(args) != 2 {
		err = errors.New("expecting 2 argument")
		return
	}
	reg, err = asReg(args[0])
	if err != nil {
		return
	}
	val, err = asValue(args[1])
	return
}

func parseAsValAndVal(args []string) (val1 value, val2 value, err error) {
	if len(args) != 2 {
		err = errors.New("expecting 2 argument")
		return
	}
	val1, err = asValue(args[0])
	if err != nil {
		return
	}
	val2, err = asValue(args[1])
	return
}
