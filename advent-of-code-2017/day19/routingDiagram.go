package main

import "strings"

const EMPTY_TILE = ' '
const PATH_TILES = "|-+"

type RoutingDiagram struct {
	grid [][]rune
}

func NewRoutingDiagram(input string) RoutingDiagram {
	lines := strings.Split(input, "\n")

	grid := make([][]rune, 0, len(lines))

	for _, line := range lines {
		if len(line) == 0 {
			continue
		}

		row := make([]rune, 0, len(line))

		for _, c := range line {
			row = append(row, c)
		}

		grid = append(grid, row)
	}

	return RoutingDiagram{grid}
}

func (rd *RoutingDiagram) Get(pos Vec2) rune {
	if pos.x < 0 || pos.y < 0 || pos.y >= len(rd.grid) || pos.x >= len(rd.grid[pos.y]) {
		return EMPTY_TILE
	}
	return rd.grid[pos.y][pos.x]
}

func (rd *RoutingDiagram) IsAccessible(pos Vec2) bool {
	return rd.Get(pos) != EMPTY_TILE
}

func (rd *RoutingDiagram) IsLetter(pos Vec2) bool {
	return rd.IsAccessible(pos) &&
		!strings.ContainsRune(PATH_TILES, rd.Get(pos))
}
