package day10

import (
	"context"
	"sort"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/stack"
)

var Solution = aoc.NewDay(part1, part2)

const (
	openingBrackets = "([{<"
	closingBrackets = ")]}>"
)

func closingBracket(opening rune) rune {
	switch opening {
	case '(':
		return ')'
	case '[':
		return ']'
	case '{':
		return '}'
	case '<':
		return '>'
	}
	return ' '
}

func part1(ctx context.Context, input string) (interface{}, error) {
	syntaxErrorScore := 0

	bracketErrorScore := map[rune]int{
		')': 3,
		']': 57,
		'}': 1197,
		'>': 25137,
	}

lines:
	for _, line := range strings.Split(input, "\n") {
		chunkStack := stack.New[rune]()

		for _, r := range line {
			if strings.Contains(openingBrackets, string(r)) {
				chunkStack.Push(r)
			}
			if strings.Contains(closingBrackets, string(r)) {
				if chunkStack.IsEmpty() {
					// found a closing bracket without matching opening bracket
					syntaxErrorScore += bracketErrorScore[r]
					continue lines
				}

				lastR := chunkStack.Pop()
				if r != closingBracket(lastR) {
					// opening and closing bracket are not of the same type
					syntaxErrorScore += bracketErrorScore[r]
					continue lines
				}
			}
		}
	}

	return syntaxErrorScore, nil
}

func part2(ctx context.Context, input string) (interface{}, error) {
	var syntaxCompletionScores []int

	bracketCompletionScore := map[rune]int{
		')': 1,
		']': 2,
		'}': 3,
		'>': 4,
	}

lines:
	for _, line := range strings.Split(input, "\n") {
		chunkStack := stack.New[rune]()

		for _, r := range line {
			if strings.Contains(openingBrackets, string(r)) {
				chunkStack.Push(r)
			}
			if strings.Contains(closingBrackets, string(r)) {
				if chunkStack.IsEmpty() {
					// found a closing bracket without matching opening bracket
					continue lines
				}

				lastR := chunkStack.Pop()
				if r != closingBracket(lastR) {
					// opening and closing bracket are not of the same type
					continue lines
				}
			}
		}

		syntaxCompletionScore := 0
		for !chunkStack.IsEmpty() {
			neededBracket := closingBracket(chunkStack.Pop())

			syntaxCompletionScore *= 5
			syntaxCompletionScore += bracketCompletionScore[neededBracket]
		}

		syntaxCompletionScores = append(syntaxCompletionScores, syntaxCompletionScore)
	}

	sort.Ints(syntaxCompletionScores)
	return syntaxCompletionScores[len(syntaxCompletionScores)/2], nil
}
