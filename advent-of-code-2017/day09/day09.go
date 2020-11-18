package day09

func SolvePart1(input string) interface{} {
	score, _ := processTheGarbageStream(input)
	return score
}

func SolvePart2(input string) interface{} {
	_, garbageCount := processTheGarbageStream(input)
	return garbageCount
}

func processTheGarbageStream(s string) (score, garbageCount int) {
	chars := []rune(s)

	nesting := 0
	inGarbage := false
	ignoreNext := false

	for _, c := range chars {
		if ignoreNext {
			ignoreNext = false
			continue
		}

		if inGarbage && c != '!' && c != '>' {
			garbageCount += 1
		}

		switch c {
		case '{':
			if !inGarbage {
				nesting += 1
				score += nesting
			}
		case '}':
			if !inGarbage {
				nesting -= 1
			}

		case '<':
			inGarbage = true
		case '>':
			inGarbage = false

		case '!':
			ignoreNext = true

		default:
		}
	}
	return
}
