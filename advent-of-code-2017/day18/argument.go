package day18

import (
	"fmt"
	"strconv"
)

func parseRegister(s string) Register {
	if len(s) != 1 {
		panic(fmt.Sprintf("could not parse argument %q as register: length is not 1", s))
	}
	return Register([]rune(s)[0])
}

// Value can either be a constant number or a reference to a number stored in a
// register that should be resolved at run-time.
type Value interface {
	resolve(c *Controller) int
}

func parseValue(s string) Value {
	i, err := strconv.Atoi(s)
	if err == nil {
		return ValueNumber(i)
	}
	return ValueRegister(parseRegister(s))
}

type ValueNumber int

func (v ValueNumber) resolve(c *Controller) int {
	return int(v)
}

type ValueRegister Register

func (v ValueRegister) resolve(c *Controller) int {
	return c.get(Register(v))
}
