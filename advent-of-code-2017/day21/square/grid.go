package square

import (
	"strings"
)

// Compile-time proof of interface implementation
var _ GridLike = (Grid)(nil)

type Grid [][]rune

func (g Grid) String() string {
	return String(g)
}

func ParseGrid(input string) (g Grid) {
	for _, line := range strings.Split(input, "/") {
		g = append(g, []rune(line))
	}
	return g
}

func (g Grid) Get(x, y int) rune {
	return g[y][x]
}

func (g Grid) GetRow(y int) []rune {
	return g[y]
}

func (g Grid) Size() int {
	return len(g)
}
