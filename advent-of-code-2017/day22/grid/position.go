package grid

type Pos struct {
	X, Y int
}

func NewPos(x, y int) Pos {
	return Pos{x, y}
}

func Step(p Pos, dir Dir) Pos {
	switch dir {
	case North:
		return Pos{p.X, p.Y - 1}
	case South:
		return Pos{p.X, p.Y + 1}
	case East:
		return Pos{p.X + 1, p.Y}
	case West:
		return Pos{p.X - 1, p.Y}
	}
	return p
}
