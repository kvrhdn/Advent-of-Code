package grid

type InfiniteGrid map[Pos]int

func New() InfiniteGrid {
	return make(map[Pos]int)
}

func (g *InfiniteGrid) Get(pos Pos) int {
	return (*g)[pos]
}

func (g *InfiniteGrid) Set(pos Pos, value int) {
	(*g)[pos] = value
}

func (g *InfiniteGrid) Center() Pos {
	var minX, maxX, minY, maxY int

	for p := range *g {
		minX = min(minX, p.X)
		maxX = max(maxX, p.X)
		minY = min(minY, p.Y)
		maxY = max(maxY, p.Y)
	}

	return Pos{(minX + maxX) / 2, (minY + maxY) / 2}
}

func min(v1, v2 int) int {
	if v2 < v1 {
		return v2
	}
	return v1
}

func max(v1, v2 int) int {
	if v2 > v1 {
		return v2
	}
	return v1
}
