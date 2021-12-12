package day12

import (
	"context"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/set"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

type connectionsMap map[string][]string

func generator(ctx context.Context, input string) (connectionsMap, error) {
	connections := make(connectionsMap)

	for _, line := range strings.Split(input, "\n") {
		caves := strings.Split(line, "-")

		caveA := caves[0]
		caveB := caves[1]

		connectionsA := connections[caveA]
		connectionsA = append(connectionsA, caveB)
		connections[caveA] = connectionsA

		connectionsB := connections[caveB]
		connectionsB = append(connectionsB, caveA)
		connections[caveB] = connectionsB
	}

	return connections, nil
}

func isLowercase(s string) bool {
	return s == strings.ToLower(s)
}

func findPaths(connections connectionsMap, pathSoFar []string, visitedSmallCaves set.Set[string], repeatedSmallCaveVisits int) [][]string {
	var paths [][]string

	currentCave := pathSoFar[len(pathSoFar)-1]

	for _, connectedCave := range connections[currentCave] {
		// can't go back to start
		if connectedCave == "start" {
			continue
		}

		// we made it!
		if connectedCave == "end" {
			path := append(pathSoFar, connectedCave)
			paths = append(paths, path)
			continue
		}

		// take a copy for every branch
		visitedSmallCaves := visitedSmallCaves.Copy()

		if isLowercase(connectedCave) {
			// can't visit a small cave twice
			if visitedSmallCaves.Contains(connectedCave) {
				// except if repeatedSmallCaveVisits is not 0!
				if repeatedSmallCaveVisits > 0 {
					newPaths := findPaths(connections, append(pathSoFar, connectedCave), visitedSmallCaves, repeatedSmallCaveVisits-1)
					paths = append(paths, newPaths...)
				}
				continue
			}
			visitedSmallCaves.Add(connectedCave)
		}

		newPaths := findPaths(connections, append(pathSoFar, connectedCave), visitedSmallCaves, repeatedSmallCaveVisits)
		paths = append(paths, newPaths...)
	}

	return paths
}

func part1(ctx context.Context, input connectionsMap) (interface{}, error) {
	paths := findPaths(input, []string{"start"}, set.New[string](), 0)
	return len(paths), nil
}

func part2(ctx context.Context, input connectionsMap) (interface{}, error) {
	paths := findPaths(input, []string{"start"}, set.New[string](), 1)
	return len(paths), nil
}
