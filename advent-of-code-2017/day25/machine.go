package day25

type Direction int

const (
	Right Direction = 1
	Left  Direction = -1
)

type State struct {
	ifZero Action
	ifOne  Action
}

type Action struct {
	write     int
	move      Direction
	nextState rune
}

func action(write int, move Direction, nextState rune) Action {
	return Action{
		write:     write,
		move:      move,
		nextState: nextState,
	}
}
