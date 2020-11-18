package grid

import (
	"fmt"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day10/knotHash"
)

type Grid struct {
	d [128][128]int
}

func InitGridKnotHash(input string) (g Grid) {
	for r := 0; r < 128; r++ {
		hash := knotHash.DenseKnotHash(fmt.Sprintf("%v-%v", input, r))

		// a hash contains 128 bits encoded as 32 chars of 4 bits each
		for i, c := range []rune(hash) {
			var value int

			_, err := fmt.Sscanf(string(c), "%x", &value)
			if err != nil {
				panic(err)
			}

			g.d[r][(4*i)+0] = (value & 0x8) / 8
			g.d[r][(4*i)+1] = (value & 0x4) / 4
			g.d[r][(4*i)+2] = (value & 0x2) / 2
			g.d[r][(4*i)+3] = (value & 0x1) / 1
		}
	}

	return g
}

func (g *Grid) Regionalize() (regions int) {
	nextRegion := 1

	for i := range g.d {
		for j := range g.d[i] {

			sq := g.get(i, j)

			if *sq == 0 {
				continue
			}

			regionAbove := g.getValue(i-1, j)
			regionLeft := g.getValue(i, j-1)

			if regionAbove == 0 && regionLeft == 0 {
				*sq = nextRegion
				nextRegion += 1

				regions += 1
				continue
			}
			if regionAbove != 0 && regionLeft == 0 {
				*sq = regionAbove
				continue
			}
			if regionAbove == 0 && regionLeft != 0 {
				*sq = regionLeft
				continue
			}
			if regionAbove == regionLeft {
				*sq = regionAbove
				continue
			}

			mergeInto := min(regionAbove, regionLeft)
			mergeFrom := max(regionAbove, regionLeft)

			*sq = mergeInto

			for i2 := 0; i2 <= i; i2++ {
				for j2 := range g.d[i2] {
					if g.getValue(i2, j2) == mergeFrom {
						*g.get(i2, j2) = mergeInto
					}
				}
			}

			regions -= 1
		}
	}
	return
}

func (g *Grid) SquaresOccupied() int {
	count := 0

	for _, row := range g.d {
		for _, sq := range row {
			if sq > 0 {
				count += 1
			}
		}
	}

	return count
}

func (g *Grid) get(row, col int) *int {
	return &(g.d[row][col])
}

func (g *Grid) getValue(row, col int) int {
	if row < 0 || col < 0 {
		return 0
	}
	return g.d[row][col]
}

func min(i int, j int) int {
	if i <= j {
		return i
	} else {
		return j
	}
}

func max(i int, j int) int {
	if i >= j {
		return i
	} else {
		return j
	}
}
