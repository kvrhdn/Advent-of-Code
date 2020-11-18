package day19

type Program struct {
	pos Vec2
	dir Vec2
}

func (p *Program) AttemptToMove(isAccessible func(pos Vec2) bool) (stuck bool) {
	possibleNewDirs := []Vec2{p.dir, rotate(p.dir), rotate(invert(p.dir))}

	for _, newDir := range possibleNewDirs {
		newPos := sum(p.pos, newDir)

		if isAccessible(newPos) {
			p.dir = newDir
			p.pos = newPos
			return
		}
	}

	stuck = true
	return
}
