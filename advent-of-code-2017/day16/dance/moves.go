package dance

import (
	"fmt"
	"strings"
)

type Move interface {
	apply([]rune) []rune
}

func ParseMoves(input string) (moves []Move) {
	for _, inputMove := range strings.Split(input, ",") {
		var move Move

		switch firstRune(inputMove) {
		case 's':
			move = parseSpin(inputMove)
		case 'x':
			move = parseExchange(inputMove)
		case 'p':
			move = parsePartner(inputMove)
		default:
			panic(fmt.Sprintf("unrecognized dance move: %v", inputMove))
		}

		moves = append(moves, move)
	}
	return
}

func firstRune(s string) rune {
	return []rune(s)[0]
}

type spin struct {
	size int
}

func parseSpin(str string) (s spin) {
	_, err := fmt.Sscanf(str, "s%d", &s.size)
	if err != nil {
		panic(fmt.Sprintf("could not parse spin from %v", str))
	}
	return
}

func (s spin) apply(p []rune) []rune {
	i := len(p) - s.size
	return append(p[i:], p[:i]...)
}

type exchange struct {
	positionA, positionB int
}

func parseExchange(str string) (e exchange) {
	_, err := fmt.Sscanf(str, "x%d/%d", &e.positionA, &e.positionB)
	if err != nil {
		panic(fmt.Sprintf("could not parse exchange from %v", str))
	}
	return
}

func (e exchange) apply(p []rune) []rune {
	p[e.positionA], p[e.positionB] = p[e.positionB], p[e.positionA]
	return p
}

type partner struct {
	programA, programB rune
}

func parsePartner(str string) (p partner) {
	_, err := fmt.Sscanf(str, "p%c/%c", &p.programA, &p.programB)
	if err != nil {
		panic(fmt.Sprintf("could not parse partner from %v", str))
	}
	return
}

func (p partner) apply(programs []rune) []rune {
	positionA := -1
	positionB := -1

	// we are not checking whether a program appears twice in the list
	for i, program := range programs {
		if program == p.programA {
			positionA = i
		}
		if program == p.programB {
			positionB = i
		}
	}

	if positionA < 0 || positionB < 0 {
		panic("could not apply partner dance move, could not find a program")
	}

	programs[positionA], programs[positionB] = programs[positionB], programs[positionA]
	return programs
}
