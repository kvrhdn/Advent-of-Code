package day18

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestExamplePart1(t *testing.T) {
	exampleInput := `set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2`

	assert.Equal(t, 4, SolvePart1(exampleInput))
}

func TestExamplePart2(t *testing.T) {
	exampleInput := `snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d`

	assert.Equal(t, 3, SolvePart2(exampleInput))
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 18, "../")

	assert.Equal(t, 9423, SolvePart1(input))
	assert.Equal(t, 7620, SolvePart2(input))
}
