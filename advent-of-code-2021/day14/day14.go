package day14

import (
	"context"
	"fmt"
	"math"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/linkedlist"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

type manual struct {
	polymerTemplate string
	insertionRules  map[string]string
}

func generator(ctx context.Context, input string) (manual, error) {
	paragraphs := strings.Split(input, "\n\n")

	polymerTemplate := paragraphs[0]

	insertionRules := make(map[string]string)
	for _, line := range strings.Split(paragraphs[1], "\n") {
		split := strings.Split(line, " -> ")
		insertionRules[split[0]] = split[1]
	}

	return manual{polymerTemplate, insertionRules}, nil
}

func part1(ctx context.Context, input manual) (interface{}, error) {
	head := linkedlist.New[rune](rune(input.polymerTemplate[0]))
	node := head
	for _, element := range input.polymerTemplate[1:] {
		node = node.InsertAfter(element)
	}

	for i := 0; i < 10; i++ {
		node = head
		for {
			// no element after this, stop
			if node.Next() == nil {
				break
			}

			pair := fmt.Sprintf("%c%c", node.Value(), node.Next().Value())
			newElement, ok := input.insertionRules[pair]
			if ok {
				node = node.InsertAfter(rune(newElement[0]))
			}
			node = node.Next()
		}
	}

	occurrences := make(map[rune]int)
	node = head
	for {
		occurrences[node.Value()] += 1
		if node.Next() == nil {
			break
		}
		node = node.Next()
	}

	minMaxDiff := diffMaxMin(occurrences)

	return minMaxDiff, nil
}

func part2(ctx context.Context, input manual) (interface{}, error) {
	// keep map with pairs -> count
	pairs := make(map[string]int)
	occurrences := make(map[rune]int)

	for i := 0; i < len(input.polymerTemplate)-1; i++ {
		pair := input.polymerTemplate[i : i+2]
		pairs[pair] += 1
	}

	for _, element := range input.polymerTemplate {
		occurrences[element] += 1
	}

	for i := 0; i < 40; i++ {
		newPairs := make(map[string]int)

		for pair, count := range pairs {
			newElement, ok := input.insertionRules[pair]
			if !ok {
				// no reaction, drop this pair
				continue
			}

			newPairOne := fmt.Sprintf("%c%c", pair[0], newElement[0])
			newPairTwo := fmt.Sprintf("%c%c", newElement[0], pair[1])

			newPairs[newPairOne] += count
			newPairs[newPairTwo] += count

			occurrences[rune(newElement[0])] += count
		}

		pairs = newPairs
	}

	return diffMaxMin(occurrences), nil
}

func diffMaxMin(occurrences map[rune]int) int {
	min := math.MaxInt
	max := math.MinInt

	for _, count := range occurrences {
		if count < min {
			min = count
		}
		if count > max {
			max = count
		}
	}

	return max - min
}
