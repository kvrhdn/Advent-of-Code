package day19

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/stretchr/testify/assert"
)

func TestExample(t *testing.T) {
	exampleInput := `     |
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+`

	assert.Equal(t, "ABCDEF", SolvePart1(exampleInput))
	assert.Equal(t, 38, SolvePart2(exampleInput))
}

func TestRealInput(t *testing.T) {
	input := aoc.ReadInputRelative(2017, 19, "../")

	assert.Equal(t, "FEZDNIVJWT", SolvePart1(input))
	assert.Equal(t, 17200, SolvePart2(input))
}
