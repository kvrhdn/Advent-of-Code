package day12

import (
	"fmt"
	"strconv"
	"strings"
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

func parseInput(input string) []Set {
	// parse pipe connections
	var pipeGroups []Set

	for _, line := range strings.Split(input, "\n") {
		set := newSet()

		var p int
		_, err := fmt.Sscanf(line, "%d <-> ", &p)
		if err != nil {
			panic(err)
		}

		set.insert(p)

		indexPipe := strings.Index(line, "<-> ")
		if indexPipe == -1 {
			panic("did not find <->")
		}

		programs := strings.Split(line[indexPipe+4:], ", ")
		for _, pString := range programs {
			p, err := strconv.Atoi(pString)
			if err != nil {
				panic(err)
			}
			set.insert(p)
		}

		pipeGroups = append(pipeGroups, set)
	}

	// combine groups
	var groups []Set

	for len(pipeGroups) > 0 {
		curr := pipeGroups[0]
		pipeGroups = pipeGroups[1:]

		var matches []int
		for i, group := range groups {
			if group.anyInCommon(curr) {
				matches = append(matches, i)
			}
		}

		// no matches, add a new group
		if len(matches) == 0 {
			groups = append(groups, curr)
			continue
		}

		// join new group with first match
		firstMatch := matches[0]
		groups[firstMatch].merge(curr)

		// join any other matches and remove their groups
		for i := len(matches) - 1; i > 0; i-- {
			index := matches[i]
			groups[firstMatch].merge(groups[index])
			groups = append(groups[:index], groups[index+1:]...)
		}
	}

	return groups
}
