package day15

import (
	"context"
	"fmt"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

type chitonMap struct {
	risk []int
	size int // the map is square
}

func (m *chitonMap) WeightAt(c coord) (int, bool) {
	if c.x < 0 || c.x >= m.size || c.y < 0 || c.y >= m.size {
		return 0, false
	}
	return m.risk[c.y*m.size+c.x], true
}

func generator(ctx context.Context, input string) (*chitonMap, error) {
	m := &chitonMap{}

	lines := strings.Split(input, "\n")
	m.size = len(lines)

	for _, line := range lines {
		runes := line
		if len(runes) != m.size {
			return &chitonMap{}, fmt.Errorf("map is not square, width is %d but expected %d", len(runes), m.size)
		}

		for _, r := range runes {
			risk := int(r - '0')
			m.risk = append(m.risk, risk)
		}
	}

	return m, nil
}

type repeatingChitonMap struct {
	m       *chitonMap
	repeats int
}

func (r *repeatingChitonMap) WeightAt(c coord) (int, bool) {
	max := r.m.size * r.repeats
	if c.x < 0 || c.x >= max || c.y < 0 || c.y >= max {
		return 0, false
	}

	cost, _ := r.m.WeightAt(coord{
		x: c.x % r.m.size,
		y: c.y % r.m.size,
	})

	offset := func(pos int) int {
		return pos / r.m.size
	}
	cost += offset(c.x) + offset(c.y)

	if cost >= 10 {
		// err, there is probably an easier way to express this...
		cost = (cost-(cost/10))%9 + 1
	}

	return cost, true
}

func part1(ctx context.Context, m *chitonMap) (interface{}, error) {
	end := coord{m.size - 1, m.size - 1}

	return findPathLowestCost(end, m), nil
}

func part2(ctx context.Context, m *chitonMap) (interface{}, error) {
	end := coord{5*m.size - 1, 5*m.size - 1}
	repeatingMap := &repeatingChitonMap{m, 5}

	return findPathLowestCost(end, repeatingMap), nil
}
