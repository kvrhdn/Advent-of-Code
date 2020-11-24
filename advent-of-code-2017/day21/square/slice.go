package square

import (
	"fmt"
	"strings"
)

// Compile-time proof of interface implementation
var _ GridLike = (*SquareGridSlice)(nil)

type SquareGridSlice struct {
	grid GridLike
	x, y int
	size int
}

func Slice(g GridLike, x, y, size int) *SquareGridSlice {
	if x+size > g.Size() || y+size > g.Size() {
		panic("slice is out-of-bounds")
	}

	return &SquareGridSlice{
		grid: g,
		x:    x,
		y:    y,
		size: size,
	}
}

func (s *SquareGridSlice) String() string {
	var str strings.Builder

	str.WriteString(String(s))
	str.WriteString(fmt.Sprintf("Slice: (%v, %v) size %v\n", s.x, s.y, s.size))

	return str.String()
}

func (s *SquareGridSlice) Get(x, y int) rune {
	return s.grid.Get(s.x+x, s.y+y)
}

func (s *SquareGridSlice) GetRow(y int) []rune {
	return s.grid.GetRow(s.y + y)[s.x : s.x+s.size]
}

func (s *SquareGridSlice) Size() int {
	return s.size
}
