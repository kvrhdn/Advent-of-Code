package day09

import (
	"context"
	"sort"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

type point struct {
	x, y int
}

func (p point) neighbours() []point {
	return []point{
		{p.x - 1, p.y},
		{p.x + 1, p.y},
		{p.x, p.y - 1},
		{p.x, p.y + 1},
	}
}

type heightMap struct {
	grid          []int
	width, height int
}

func (m *heightMap) get(p point) (int, bool) {
	if p.x < 0 || p.y < 0 || p.x >= m.width || p.y >= m.height {
		return 0, false
	}
	i := m.grid[p.y*m.width+p.x]
	return i, true
}

func (m *heightMap) getOr(p point, or int) int {
	height, ok := m.get(p)
	if !ok {
		return or
	}
	return height
}

func (m *heightMap) set(p point, height int) {
	m.grid[p.y*m.width+p.x] = height
}

func generator(ctx context.Context, input string) (*heightMap, error) {
	var m heightMap
	for _, line := range strings.Split(input, "\n") {
		for _, digit := range line {
			m.grid = append(m.grid, int(digit-'0'))
		}
		m.height += 1
		m.width = len(line)
	}
	return &m, nil
}

func part1(ctx context.Context, input *heightMap) (interface{}, error) {
	var riskLowPoints int

	for y := 0; y < input.height; y++ {
	iterating:
		for x := 0; x < input.width; x++ {
			p := point{x, y}
			height, _ := input.get(p)

			for _, neighbour := range p.neighbours() {
				if height >= input.getOr(neighbour, 10) {
					continue iterating
				}
			}

			riskLowPoints += height + 1
		}
	}

	return riskLowPoints, nil
}

func part2(ctx context.Context, input *heightMap) (interface{}, error) {
	// each lowPoint will correspond with one basin
	var lowPoints []point

	for y := 0; y < input.height; y++ {
	iterating:
		for x := 0; x < input.width; x++ {
			p := point{x, y}
			height, _ := input.get(p)

			for _, neighbour := range p.neighbours() {
				if height >= input.getOr(neighbour, 10) {
					continue iterating
				}
			}

			lowPoints = append(lowPoints, point{x, y})
		}
	}

	var basinSizes []int

	// for every lowPoint, map out the basin
	for _, lowPoint := range lowPoints {
		basin := map[point]struct{}{}
		pointsToCheck := []point{lowPoint}

		for len(pointsToCheck) > 0 {
			var newPoints []point

			for _, p := range pointsToCheck {
				// spread out in the four directions
				for _, neighbour := range p.neighbours() {
					if _, ok := basin[neighbour]; ok { // already visited
						continue
					}
					if input.getOr(neighbour, 9) == 9 { // 9's don't count
						continue
					}

					newPoints = append(newPoints, neighbour)
					basin[neighbour] = struct{}{}
				}
			}

			pointsToCheck = newPoints
		}

		basinSizes = append(basinSizes, len(basin))
	}

	sort.Ints(basinSizes)
	basinSizes = basinSizes[len(basinSizes)-3:]

	return basinSizes[0] * basinSizes[1] * basinSizes[2], nil
}
