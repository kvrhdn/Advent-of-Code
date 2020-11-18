package main

import (
	"github.com/koenaad/Advent-of-Code-2017/day22/grid"
	"github.com/stretchr/testify/assert"
	"testing"
)

var exampleInput = `..#
#..
...`

func Test_infectionsAfterBursts_v1(t *testing.T) {
	t.Parallel()

	g := readInput(exampleInput)
	c := &VirusCarrier{
		pos:       g.Center(),
		dir:       grid.North,
		virusImpl: virusV1,
	}

	assert.Equal(t, infectionsAfterBursts(70, g, c), 41)
	assert.Equal(t, infectionsAfterBursts(10_000-70, g, c), 5587-41)
}

func Test_infectionsAfterBursts_v2(t *testing.T) {
	t.Parallel()

	g := readInput(exampleInput)
	c := &VirusCarrier{
		pos:       g.Center(),
		dir:       grid.North,
		virusImpl: virusV2,
	}

	assert.Equal(t, infectionsAfterBursts(100, g, c), 26)
	assert.Equal(t, infectionsAfterBursts(10_000_000-100, g, c), 2_511_944-26)
}
