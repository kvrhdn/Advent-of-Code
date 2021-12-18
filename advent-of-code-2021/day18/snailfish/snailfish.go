package snailfish

import (
	"fmt"
	"math"
	"strconv"
	"unicode"
)

type Number interface {
	fmt.Stringer

	Magnitude() int

	reduceExplosions(depth int) (carryLeft, carryRight int, replaceWith Number, reduced bool)
	reduceSplits() (replaceWith Number, reduced bool)
}

func ParseNumber(s string) (Number, int) {
	if unicode.IsDigit(rune(s[0])) {
		// assumption: we only parse snailfish numbers containing single digit natural numbers
		return newLiteral(int(s[0] - '0')), 1
	}

	// we must be parsing a pair
	n := 0

	assertNextIs := func(expected byte) {
		if s[n] != expected {
			panic(fmt.Sprintf("expected '%c', got '%c'", expected, s[n]))
		}
		n += 1
	}

	assertNextIs('[')

	left, consumed := ParseNumber(s[n:])
	n += consumed

	assertNextIs(',')

	right, consumed := ParseNumber(s[n:])
	n += consumed

	assertNextIs(']')

	return newPair(left, right), n
}

func Add(numbers ...string) Number {
	sum, _ := ParseNumber(numbers[0])
	for _, n := range numbers[1:] {
		number, _ := ParseNumber(n)
		sum = fullyReduce(newPair(sum, number))
	}
	return sum
}

func fullyReduce(n Number) Number {
reducing:
	_, _, _, reduced := n.reduceExplosions(0)
	if reduced {
		goto reducing
	}

	_, reduced = n.reduceSplits()
	if reduced {
		goto reducing
	}

	return n
}

type pair struct {
	left  Number
	right Number
}

func newPair(n1, n2 Number) Number {
	return &pair{n1, n2}
}

func (p *pair) String() string {
	return fmt.Sprintf("[%s,%s]", p.left, p.right)
}

func (p *pair) Magnitude() int {
	return 3*p.left.Magnitude() + 2*p.right.Magnitude()
}

func (p *pair) reduceExplosions(depth int) (carryLeft, carryRight int, replaceWith Number, reduced bool) {
	if depth >= 4 {
		// this pair explodes
		// assumption: exploding pairs will always contain two literals
		literalLeft := p.left.(*literal)
		literalRight := p.right.(*literal)

		return literalLeft.value, literalRight.value, newLiteral(0), true
	}

	carryLeft, carryRight, replaceWith, reduced = p.left.reduceExplosions(depth + 1)
	if reduced {
		if replaceWith != nil {
			p.left = replaceWith
		}

		// can't handle carryLeft in this pair

		if carryRight != 0 {
			if lit, ok := p.right.(*literal); ok {
				lit.value += carryRight
			} else {
				pairRight := p.right.(*pair)
				pairRight.addToLeftMostNumber(carryRight)
			}
		}

		return carryLeft, 0, nil, true
	}

	carryLeft, carryRight, replaceWith, reduced = p.right.reduceExplosions(depth + 1)
	if reduced {
		if replaceWith != nil {
			p.right = replaceWith
		}

		if carryLeft != 0 {
			if lit, ok := p.left.(*literal); ok {
				lit.value += carryLeft
			} else {
				pairLeft := p.left.(*pair)
				pairLeft.addToRightMostNumber(carryLeft)
			}
		}

		// can't handle carryRight in this pair

		return 0, carryRight, nil, true
	}

	return 0, 0, nil, false
}

func (p *pair) reduceSplits() (replaceWith Number, reduced bool) {
	replaceWith, reduced = p.left.reduceSplits()
	if reduced {
		if replaceWith != nil {
			p.left = replaceWith
		}
		return nil, true
	}

	replaceWith, reduced = p.right.reduceSplits()
	if reduced {
		if replaceWith != nil {
			p.right = replaceWith
		}
		return nil, true
	}

	return nil, false
}

func (p *pair) addToLeftMostNumber(add int) {
	if lit, ok := p.left.(*literal); ok {
		lit.value += add
	} else {
		pa := p.left.(*pair)
		pa.addToLeftMostNumber(add)
	}
}

func (p *pair) addToRightMostNumber(add int) {
	if lit, ok := p.right.(*literal); ok {
		lit.value += add
	} else {
		pa := p.right.(*pair)
		pa.addToRightMostNumber(add)
	}
}

type literal struct {
	value int
}

func newLiteral(value int) Number {
	return &literal{value}
}

func (l *literal) String() string {
	return strconv.Itoa(l.value)
}

func (l *literal) Magnitude() int {
	return l.value
}

func (l *literal) reduceExplosions(depth int) (_, _ int, _ Number, _ bool) {
	return
}

func (l *literal) reduceSplits() (replaceWith Number, reduced bool) {
	if l.value >= 10 {
		return newPair(
			newLiteral(int(math.Floor(float64(l.value)/2))),
			newLiteral(int(math.Ceil(float64(l.value)/2))),
		), true
	}
	return nil, false
}
