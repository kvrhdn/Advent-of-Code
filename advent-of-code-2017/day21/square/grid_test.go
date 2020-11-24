package square

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParseGrid(t *testing.T) {
	grid := ParseGrid(".#./..#/###")
	expected := Grid([][]rune{
		{'.', '#', '.'},
		{'.', '.', '#'},
		{'#', '#', '#'},
	})

	assert.Equal(t, expected, grid)
	assert.True(t, Equals(grid, expected))
}
