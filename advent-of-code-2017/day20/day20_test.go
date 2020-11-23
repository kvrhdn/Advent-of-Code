package day20

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestExamplePart1(t *testing.T) {
	exampleInput := `p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>`

	assert.Equal(t, 0, SolvePart1(exampleInput))
}

func TestExamplePart2(t *testing.T) {
	exampleInput := `p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>`

	assert.Equal(t, 1, SolvePart2(exampleInput))
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 20, "../")

	assert.Equal(t, 170, SolvePart1(input))
	assert.Equal(t, 571, SolvePart2(input))
}
