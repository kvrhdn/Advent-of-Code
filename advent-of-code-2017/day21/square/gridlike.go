package square

import (
	"strings"
)

// GridLike represents a square 2D grid.
type GridLike interface {
	GetRow(y int) []rune
	Get(x, y int) rune
	Size() int
}

func String(g GridLike) string {
	var str strings.Builder

	for y := 0; y < g.Size(); y++ {
		str.WriteString(string(g.GetRow(y)))
		str.WriteRune('\n')
	}

	return str.String()
}

func Equals(g1, g2 GridLike) bool {
	if g1.Size() != g2.Size() {
		return false
	}

	for y := 0; y < g1.Size(); y++ {
		for x := 0; x < g1.Size(); x++ {
			if g1.Get(x, y) != g2.Get(x, y) {
				return false
			}
		}
	}

	return true
}

func Count(g GridLike, r rune) (count int) {
	for y := 0; y < g.Size(); y++ {
		for x := 0; x < g.Size(); x++ {
			if g.Get(x, y) == r {
				count++
			}
		}
	}
	return
}

func Divide(g GridLike, size int) []GridLike {
	if g.Size()%size != 0 {
		panic("can't divide")
	}

	var slices []GridLike

	for y := 0; y < g.Size(); y += size {
		for x := 0; x < g.Size(); x += size {
			slices = append(slices, Slice(g, x, y, size))
		}
	}

	return slices
}

func Combine(g []GridLike, blocks int) Grid {
	blockWidth := g[0].Size()

	var grid [][]rune

	for yBlock := 0; yBlock < blocks; yBlock++ {
		for innerY := 0; innerY < blockWidth; innerY++ {
			row := make([]rune, blocks*blockWidth)

			for xBlock := 0; xBlock < blocks; xBlock++ {
				for innerX := 0; innerX < blockWidth; innerX++ {

					row[(xBlock*blockWidth)+innerX] = g[(yBlock*blocks)+xBlock].Get(innerX, innerY)

				}
			}

			grid = append(grid, row)
		}
	}

	return grid
}

// Transposed creates a transposed copy of the given grid.
func Transposed(grid GridLike) (new Grid) {
	size := grid.Size()

	for y := size - 1; y >= 0; y-- {
		var row []rune

		for x := size - 1; x >= 0; x-- {
			row = append(row, grid.Get(y, x))
		}

		new = append(new, row)
	}

	return new
}

// Flipped creates a new grid, horizontally flipped.
func Flipped(grid GridLike) (new Grid) {
	for y := grid.Size() - 1; y >= 0; y-- {
		new = append(new, grid.GetRow(y))
	}
	return
}
