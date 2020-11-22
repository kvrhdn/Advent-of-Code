package day12

import (
	"regexp"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/shared/intslice"
)

func SolvePart1(input string) interface{} {
	groups := parseInput(input)

	for _, group := range groups {
		if group.contains(0) {
			return len(group)
		}
	}

	panic("did not find program 0")
}

func SolvePart2(input string) interface{} {
	groups := parseInput(input)
	return len(groups)
}

func parseInput(input string) (groups []Set) {
	regex := regexp.MustCompile(`(\d+)`)

	for _, line := range strings.Split(input, "\n") {
		programs := intslice.Atoi(regex.FindAllString(line, -1))

		// look for groups that are connected to this pipe
		var matches []int
		for i, group := range groups {
			if group.containAnyOf(programs) {
				matches = append(matches, i)
			}
		}

		// no matches, add a new group
		if len(matches) == 0 {
			groups = append(groups, newSetFromSlice(programs))
			continue
		}

		// merge new programs into the first match
		firstMatch := matches[0]
		groups[firstMatch].insertAll(programs)

		// merge the remaining matches and remove their groups
		for i := len(matches) - 1; i > 0; i-- {
			index := matches[i]
			groups[firstMatch].merge(groups[index])
			groups = append(groups[:index], groups[index+1:]...)
		}
	}
	return
}
