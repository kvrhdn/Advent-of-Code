package day14

import (
	"testing"
)

func TestSolution(t *testing.T) {
	Solution.VerifySolution(t, 14, 2988, 3572761917024)
}

func TestExample(t *testing.T) {
	example := `NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C`
	Solution.VerifyInput(t, example, 1588, 2188189693529)
}
