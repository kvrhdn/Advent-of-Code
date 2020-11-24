package day21

import (
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day21/square"
)

type PatternMatcher struct {
	patterns []square.Grid
	size     int
	output   square.Grid
}

func parsePattern(input string) PatternMatcher {
	segments := strings.Split(input, " => ")

	grid := square.ParseGrid(segments[0])
	size := len(grid)

	var patterns []square.Grid

	// enhancement: we are storing a ton of duplicates, we could filter these out
	patterns = append(patterns, grid) // original
	grid = square.Transposed(grid)
	patterns = append(patterns, grid)
	grid = square.Flipped(grid)
	patterns = append(patterns, grid) // rot 90°
	grid = square.Transposed(grid)
	patterns = append(patterns, grid)
	grid = square.Flipped(grid)
	patterns = append(patterns, grid) // rot 180°
	grid = square.Transposed(grid)
	patterns = append(patterns, grid)
	grid = square.Flipped(grid)
	patterns = append(patterns, grid) // rot 270°
	grid = square.Transposed(grid)
	patterns = append(patterns, grid)

	output := square.ParseGrid(segments[1])

	return PatternMatcher{
		patterns: patterns,
		size:     size,
		output:   output,
	}
}

func (p *PatternMatcher) matches(grid square.GridLike) bool {
	if p.size != grid.Size() {
		return false
	}

	for _, pattern := range p.patterns {
		if square.Equals(pattern, grid) {
			return true
		}
	}

	return false
}
