package day04

import (
	"context"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/ints"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

type puzzleInput struct {
	numbers []int
	boards  []board
}

type numberDrawer struct {
	next  []int
	drawn map[int]struct{}
}

func newNumberDrawer(numbers []int) *numberDrawer {
	return &numberDrawer{
		next:  numbers,
		drawn: make(map[int]struct{}),
	}
}

func (n *numberDrawer) Draw() int {
	number := n.next[0]
	n.drawn[number] = struct{}{}
	n.next = n.next[1:]
	return number
}

func (n *numberDrawer) IsDrawn(num int) bool {
	_, ok := n.drawn[num]
	return ok
}

type board struct {
	values        []int // store 2D values in a consecutive array
	width, height int
}

func (b *board) Row(r int, fn func(i int)) {
	for _, v := range b.values[b.width*r : b.width*(r+1)] {
		fn(v)
	}
}

func (b *board) Column(c int, fn func(v int)) {
	for i := 0; i < b.height; i++ {
		fn(b.values[i*b.width+c])
	}
}

func (b *board) All(fn func(v int)) {
	for _, v := range b.values {
		fn(v)
	}
}

func (b *board) HasBingo(numberDrawer *numberDrawer) bool {
	// check the rows
	for i := 0; i < b.height; i++ {
		count := 0
		b.Row(i, func(v int) {
			if numberDrawer.IsDrawn(v) {
				count++
			}
		})
		if count == b.height {
			return true
		}
	}

	// check the columns
	for i := 0; i < b.width; i++ {
		count := 0
		b.Column(i, func(v int) {
			if numberDrawer.IsDrawn(v) {
				count++
			}
		})
		if count == b.width {
			return true
		}
	}

	return false
}

func generator(ctx context.Context, input string) (puzzleInput, error) {
	paragraphs := strings.Split(input, "\n\n")

	numbers, err := ints.Parse(paragraphs[0], func(s string) []string {
		return strings.Split(s, ",")
	})
	if err != nil {
		return puzzleInput{}, err
	}

	var boards []board
	for _, paragraph := range paragraphs[1:] {
		b := board{}

		for _, line := range strings.Split(paragraph, "\n") {
			values, err := ints.Parse(line, strings.Fields)
			if err != nil {
				return puzzleInput{}, err
			}

			b.values = append(b.values, values...)
			b.height += 1
			b.width = len(values)
		}

		boards = append(boards, b)
	}

	return puzzleInput{numbers, boards}, nil
}

func part1(ctx context.Context, input puzzleInput) (interface{}, error) {
	numberDrawer := newNumberDrawer(input.numbers)

	// Draw the first 4 numbers, no one can have bingo yet
	for i := 0; i < 4; i++ {
		numberDrawer.Draw()
	}

	var drawnNumber int
	var winningBoard board
bingo:
	for {
		drawnNumber = numberDrawer.Draw()

		for _, b := range input.boards {
			if b.HasBingo(numberDrawer) {
				winningBoard = b
				break bingo
			}
		}
	}

	sumRemaining := 0
	winningBoard.All(func(v int) {
		if !numberDrawer.IsDrawn(v) {
			sumRemaining += v
		}
	})

	return sumRemaining * drawnNumber, nil
}

func part2(ctx context.Context, input puzzleInput) (interface{}, error) {
	numberDrawer := newNumberDrawer(input.numbers)

	// Draw the first 4 numbers, no one can have bingo yet
	for i := 0; i < 4; i++ {
		numberDrawer.Draw()
	}

	boards := input.boards

	var drawnNumber int
	var lastWinningBoard board
bingo:
	for {
		drawnNumber = numberDrawer.Draw()

		var remainingBoards []board

	checkBoards:
		for _, b := range boards {
			if b.HasBingo(numberDrawer) {
				lastWinningBoard = b
				continue checkBoards
			}

			// not a winner yet
			remainingBoards = append(remainingBoards, b)
		}

		boards = remainingBoards

		if len(boards) == 0 {
			break bingo
		}
	}

	sumRemaining := 0
	lastWinningBoard.All(func(v int) {
		if !numberDrawer.IsDrawn(v) {
			sumRemaining += v
		}
	})

	return sumRemaining * drawnNumber, nil
}
