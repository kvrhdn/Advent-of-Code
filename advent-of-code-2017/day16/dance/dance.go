package dance

func Dance(programsString string, moves []Move) string {
	programs := []rune(programsString)

	for _, move := range moves {
		programs = move.apply(programs)
	}

	return string(programs)
}
