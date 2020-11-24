package square

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestGridSlice(t *testing.T) {
	grid := ParseGrid("#..#/#.#./.##./.##.")

	slice1 := Slice(grid, 0, 0, 4)
	assert.True(t, Equals(grid, slice1))

	slice2 := Slice(slice1, 1, 1, 2)

	assert.Equal(t, '.', slice2.Get(0, 0))
	assert.Equal(t, '#', slice2.Get(1, 0))
	assert.Equal(t, '#', slice2.Get(0, 1))
	assert.Equal(t, '#', slice2.Get(1, 1))

	assert.True(t, Equals(Slice(grid, 0, 0, 2), Slice(grid, 2, 2, 2)))

	assert.Panics(t, func() { Slice(grid, 2, 0, 3) })
	assert.Panics(t, func() { Slice(grid, 0, 2, 3) })
}
