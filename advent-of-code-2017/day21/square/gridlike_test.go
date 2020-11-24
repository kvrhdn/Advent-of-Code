package square

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestStringer(t *testing.T) {
	grid := ParseGrid("#../#.#/.##")
	expected := `#..
#.#
.##
`

	assert.Equal(t, expected, grid.String())
	assert.Equal(t, expected, fmt.Sprintf("%v", grid))

	slice := Slice(grid, 0, 0, 3)
	expected = `#..
#.#
.##
Slice: (0, 0) size 3
`

	assert.Equal(t, expected, slice.String())
	assert.Equal(t, expected, fmt.Sprintf("%v", slice))
}

func TestGridLikeDivideAndCombine(t *testing.T) {
	grid := ParseGrid("#./.#")

	slices := Divide(grid, 1)

	assert.Equal(t, '#', slices[0].Get(0, 0))
	assert.Equal(t, '.', slices[1].Get(0, 0))
	assert.Equal(t, '.', slices[2].Get(0, 0))
	assert.Equal(t, '#', slices[3].Get(0, 0))

	newGrid := Combine(slices, 2)

	assert.Equal(t, 2, newGrid.Size())
	assert.Equal(t, '#', newGrid.Get(0, 0))
	assert.Equal(t, '.', newGrid.Get(1, 0))
	assert.Equal(t, '.', newGrid.Get(0, 1))
	assert.Equal(t, '#', newGrid.Get(1, 1))
}

func TestSquareGridTransposed(t *testing.T) {
	grid := ParseGrid("#../#.#/.##")
	expected := ParseGrid("##./#../.##")

	assert.Equal(t, expected, Transposed(grid))
}

func TestSquareGridFlip(t *testing.T) {
	grid := ParseGrid("#../#.#/.##")
	expected := ParseGrid(".##/#.#/#..")

	assert.Equal(t, Grid(expected), Flipped(grid))
}
