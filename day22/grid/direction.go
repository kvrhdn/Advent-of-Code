package grid

type Dir int

const (
	North Dir = iota
	East
	South
	West
)

func LeftOf(dir Dir) Dir {
	return (dir + 3) % 4
}

func RightOf(dir Dir) Dir {
	return (dir + 1) % 4
}

func ReverseOf(dir Dir) Dir {
	return (dir + 2) % 4
}
