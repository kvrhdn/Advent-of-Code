package day22

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day22/grid"

	"github.com/stretchr/testify/assert"
)

const exampleInput = `..#
#..
...`

func TestExamplePart(t *testing.T) {
	t.Parallel()

	g := parseInput(exampleInput)
	c := &VirusCarrier{
		pos:       g.Center(),
		dir:       grid.North,
		virusImpl: virusV1,
	}
	assert.Equal(t, 41, infectionsAfterBursts(70, g, c))

	assert.Equal(t, 5587, SolvePart1(exampleInput))
}

func TestExamplePart2(t *testing.T) {
	t.Parallel()

	g := parseInput(exampleInput)
	c := &VirusCarrier{
		pos:       g.Center(),
		dir:       grid.North,
		virusImpl: virusV2,
	}
	assert.Equal(t, 26, infectionsAfterBursts(100, g, c))

	assert.Equal(t, 2511944, SolvePart2(exampleInput))
}

func TestRealInput(t *testing.T) {
	t.Parallel()

	input := aoc.ReadInputRelative(2017, 22, "../")

	assert.Equal(t, 5305, SolvePart1(input))
	assert.Equal(t, 2511424, SolvePart2(input))
}
