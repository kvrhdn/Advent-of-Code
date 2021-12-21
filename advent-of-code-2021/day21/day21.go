package day21

import (
	"context"
	"fmt"
	"strconv"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

func generator(ctx context.Context, input string) ([2]int, error) {
	var positions [2]int
	for i, line := range strings.Split(input, "\n") {
		fields := strings.Fields(line)
		position, err := strconv.Atoi(fields[len(fields)-1])
		if err != nil {
			return [2]int{}, err
		}
		positions[i] = position
	}
	return positions, nil
}

type deterministicDie struct {
	last        int
	timesRolled int
}

func (d *deterministicDie) rollThreeTimes() int {
	value := 3*d.last + 1 + 2 + 3
	d.last += 3
	d.timesRolled += 3
	return value
}

func part1(ctx context.Context, input [2]int) (interface{}, error) {
	positions := input
	var scores [2]int

	var d deterministicDie

	for {
		for p := range positions {
			roll := d.rollThreeTimes()

			positions[p] = (positions[p] + roll) % 10

			score := positions[p]
			if score == 0 {
				score = 10
			}
			scores[p] += score

			if scores[p] >= 1000 {
				return scores[(p+1)%2] * d.timesRolled, nil
			}
		}
	}
}

func memoizationKey(positions, scores [2]int, playersTurn int) string {
	return fmt.Sprintf("%d-%d,%d-%d,%d", positions[0], positions[1], scores[0], scores[1], playersTurn)
}

func playQuantumGame(memoization map[string][2]int, rolls []int, positions [2]int, scores [2]int, playersTurn int) (wins [2]int) {
	key := memoizationKey(positions, scores, playersTurn)
	if wins, ok := memoization[key]; ok {
		return wins
	}

	for _, roll := range rolls {
		// take a copy
		newPositions := positions
		newScores := scores

		newPositions[playersTurn] = (newPositions[playersTurn] + roll) % 10

		score := newPositions[playersTurn]
		if score == 0 {
			score = 10
		}
		newScores[playersTurn] += score

		if newScores[playersTurn] >= 21 {
			wins[playersTurn] += 1
			continue
		}

		newWins := playQuantumGame(memoization, rolls, newPositions, newScores, (playersTurn+1)%2)
		wins[0] += newWins[0]
		wins[1] += newWins[1]
	}

	memoization[key] = wins
	return wins
}

func part2(ctx context.Context, input [2]int) (interface{}, error) {
	memoization := make(map[string][2]int)

	var rolls []int
	for _, d1 := range []int{1, 2, 3} {
		for _, d2 := range []int{1, 2, 3} {
			for _, d3 := range []int{1, 2, 3} {
				rolls = append(rolls, d1+d2+d3)
			}
		}
	}

	wins := playQuantumGame(memoization, rolls, input, [2]int{}, 0)

	var winner int
	for _, w := range wins {
		if w > winner {
			winner = w
		}
	}

	return winner, nil
}
