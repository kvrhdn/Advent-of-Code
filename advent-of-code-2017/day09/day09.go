package day09

func SolvePart1(input string) interface{} {
	score, _ := processGarbageStream(input)
	return score
}

func SolvePart2(input string) interface{} {
	_, garbageCount := processGarbageStream(input)
	return garbageCount
}

func processGarbageStream(s string) (score, garbageCount int) {
	nesting := 0
	inGarbage := false
	ignoreNext := false

	for _, c := range []rune(s) {
		if ignoreNext {
			ignoreNext = false
			continue
		}
		if c == '!' {
			ignoreNext = true
			continue
		}

		if inGarbage {
			if c == '>' {
				inGarbage = false
				continue
			}

			garbageCount += 1
			continue
		}

		// not in garbage
		if c == '{' {
			nesting += 1
			score += nesting
			continue
		}
		if c == '}' {
			nesting -= 1
			continue
		}
		if c == '<' {
			inGarbage = true
		}
	}

	return
}
