package day14

// Grid of 128 by 128 cells. If a cell contains 0 it is empty.
type Grid [128][128]int

// get the value stored at the given row and col or 0 if out-of-bounds.
func (g *Grid) get(row, col int) int {
	if row < 0 || row >= 128 || col < 0 || col >= 128 {
		return 0
	}
	return g[row][col]
}

func (g *Grid) clusterRegions() (regions int) {
	nextRegionId := 1

	for row := range g {
		for col := range g[row] {
			if g[row][col] == 0 {
				continue
			}

			regionAbove := g.get(row-1, col)
			regionLeft := g.get(row, col-1)

			if regionAbove == 0 && regionLeft == 0 {
				g[row][col] = nextRegionId
				nextRegionId++

				regions += 1
				continue
			}
			if regionAbove != 0 && regionLeft == 0 {
				g[row][col] = regionAbove
				continue
			}
			if regionAbove == 0 && regionLeft != 0 {
				g[row][col] = regionLeft
				continue
			}
			if regionAbove == regionLeft {
				g[row][col] = regionAbove
				continue
			}

			// region above and left have a different ID, we should merge them now
			g[row][col] = regionAbove

			// change all cells with id regionLeft into regionAbove
			for row2 := 0; row2 <= row; row2++ {
				for col2 := range g[row2] {
					if g[row2][col2] == regionLeft {
						g[row2][col2] = regionAbove
					}
				}
			}

			regions -= 1
		}
	}

	return regions
}
