package day16

import (
	"fmt"
)

type DanceMove interface {
	apply(*[]rune)
}

func parseDanceMove(input string) DanceMove {
	switch input[0] {
	case 's':
		return parseSpin(input)
	case 'x':
		return parseExchange(input)
	case 'p':
		return parsePartner(input)
	default:
		panic(fmt.Sprintf("Uncrecognized move: %v", input))
	}
}

type Spin struct {
	size int
}

func parseSpin(str string) (s Spin) {
	_, err := fmt.Sscanf(str, "s%d", &s.size)
	if err != nil {
		panic(err)
	}
	return
}

func (s Spin) apply(programs *[]rune) {
	i := len(*programs) - s.size
	*programs = append((*programs)[i:], (*programs)[:i]...)
}

type Exchange struct {
	positionA, positionB int
}

func parseExchange(str string) (e Exchange) {
	_, err := fmt.Sscanf(str, "x%d/%d", &e.positionA, &e.positionB)
	if err != nil {
		panic(err)
	}
	return
}

func (e Exchange) apply(programs *[]rune) {
	(*programs)[e.positionA], (*programs)[e.positionB] = (*programs)[e.positionB], (*programs)[e.positionA]
}

type Partner struct {
	programA, programB rune
}

func parsePartner(str string) (p Partner) {
	_, err := fmt.Sscanf(str, "p%c/%c", &p.programA, &p.programB)
	if err != nil {
		panic(err)
	}
	return
}

func (p Partner) apply(programs *[]rune) {
	positionA := -1
	positionB := -1

	// assumption: a program can not be present twice in the same list
	for i, program := range *programs {
		if program == p.programA {
			positionA = i
		}
		if program == p.programB {
			positionB = i
		}
	}

	if positionA == -1 || positionB == -1 {
		panic("could not apply Partner dance move, could not find a program")
	}

	(*programs)[positionA], (*programs)[positionB] = (*programs)[positionB], (*programs)[positionA]
}
