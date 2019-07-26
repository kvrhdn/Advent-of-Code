package util

type Pos struct {
	X, Y int
}

func ManhattenDistance(pos1, pos2 Pos) int {
	return Abs(pos2.X - pos1.X) + Abs(pos2.Y - pos1.Y)
}

func Abs(value int) int {
	if value < 0 {
		return -value
	}
	return value
}
