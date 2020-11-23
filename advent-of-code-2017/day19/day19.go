package day19

func SolvePart1(input string) interface{} {
	d := parseDiagram(input)

	letters, _ := followPath(&d)

	return string(letters)
}

func SolvePart2(input string) interface{} {
	d := parseDiagram(input)

	_, steps := followPath(&d)

	return steps
}

func followPath(d *Diagram) (letters []rune, steps int) {
	pos := findStartPosition(d)
	dir := Vec2{0, 1} // start in the downwards direction
	stuck := false

	steps = 1

	for {
		pos, dir, stuck = move(pos, dir, d.isAccessible)
		if stuck {
			return letters, steps
		}

		if d.isLetter(pos) {
			letters = append(letters, d.get(pos))
		}

		steps++
	}
}

func findStartPosition(d *Diagram) Vec2 {
	for i, c := range d.grid[0] {
		if c != ' ' {
			return Vec2{i, 0}
		}
	}
	panic("could not find start position")
}

func move(pos, dir Vec2, isAccessible func(pos Vec2) bool) (newPos, newDir Vec2, isStuck bool) {
	possibleDirs := []Vec2{dir, rotateLeft(dir), rotateRight(dir)}

	for _, newDir := range possibleDirs {
		newPos = sum(pos, newDir)

		if isAccessible(newPos) {
			return newPos, newDir, false
		}
	}

	return pos, dir, true
}
