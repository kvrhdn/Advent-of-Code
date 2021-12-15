package day15

import (
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/priorityqueue"
)

type coord struct {
	x, y int
}

func (c coord) Neighbours() []coord {
	return []coord{
		{c.x - 1, c.y},
		{c.x + 1, c.y},
		{c.x, c.y - 1},
		{c.x, c.y + 1},
	}
}

type weightedMap interface {
	WeightAt(c coord) (int, bool)
}

func findPathLowestCost(end coord, m weightedMap) int {
	start := coord{0, 0}

	// positions left to scan, priority queue always returns position with the lowest cost
	frontier := priorityqueue.New[coord]()
	frontier.Push(0, start)

	costSoFar := make(map[coord]int)

	for {
		current := frontier.Pop()

		if current == end {
			return costSoFar[end]
		}

		for _, next := range current.Neighbours() {
			weigth, ok := m.WeightAt(next)
			if !ok {
				continue
			}
			newCost := costSoFar[current] + weigth

			// only update cost if not set or new is lower
			if existingCost, ok := costSoFar[next]; !ok || newCost < existingCost {
				costSoFar[next] = newCost
				frontier.Push(newCost, next)
			}
		}
	}
}
