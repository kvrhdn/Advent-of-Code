package day25

import "github.com/kvrhdn/advent-of-code/advent-of-code-2017/shared/util"

func SolvePart1(input string) interface{} {
	tape := make(map[int]int)
	cursor := 0
	state := 'A'

	steps := 12172063

	stateMachine := map[rune]State{
		'A': {
			ifZero: action(1, Right, 'B'),
			ifOne:  action(0, Left, 'C'),
		},
		'B': {
			ifZero: action(1, Left, 'A'),
			ifOne:  action(1, Left, 'D'),
		},
		'C': {
			ifZero: action(1, Right, 'D'),
			ifOne:  action(0, Right, 'C'),
		},
		'D': {
			ifZero: action(0, Left, 'B'),
			ifOne:  action(0, Right, 'E'),
		},
		'E': {
			ifZero: action(1, Right, 'C'),
			ifOne:  action(1, Left, 'F'),
		},
		'F': {
			ifZero: action(1, Left, 'E'),
			ifOne:  action(1, Right, 'A'),
		},
	}

	util.Times(steps, func() {
		var action Action

		if tape[cursor] == 0 {
			action = stateMachine[state].ifZero
		} else {
			action = stateMachine[state].ifOne
		}

		tape[cursor] = action.write
		cursor += int(action.move)
		state = action.nextState
	})

	return diagnosticsChecksum(tape)
}

func diagnosticsChecksum(tape map[int]int) (checksum int) {
	for _, v := range tape {
		if v == 1 {
			checksum++
		}
	}
	return
}
