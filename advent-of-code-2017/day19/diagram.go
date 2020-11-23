package day19

import (
	"strings"
)

const emptyTile = ' '
const pathTiles = "|-+"

type Diagram struct {
	grid [][]rune
}

func parseDiagram(input string) (d Diagram) {
	for _, line := range strings.Split(input, "\n") {
		row := make([]rune, len(line))

		for i, c := range line {
			row[i] = c
		}

		d.grid = append(d.grid, row)
	}
	return
}

func (d *Diagram) get(pos Vec2) rune {
	if pos.x < 0 || pos.y < 0 || pos.y >= len(d.grid) || pos.x >= len(d.grid[pos.y]) {
		return emptyTile
	}
	return d.grid[pos.y][pos.x]
}

func (d *Diagram) isAccessible(pos Vec2) bool {
	return d.get(pos) != emptyTile
}

func (d *Diagram) isLetter(pos Vec2) bool {
	return d.isAccessible(pos) &&
		!strings.ContainsRune(pathTiles, d.get(pos))
}
