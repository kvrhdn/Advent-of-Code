package aoc

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func readInput(year, day int) string {
	return ReadInputRelative(year, day, "")
}

func ReadInputRelative(year, day int, pathToProjectRoot string) string {
	bytes, err := ioutil.ReadFile(fmt.Sprintf("%s./input/%02d/day%02d.txt", pathToProjectRoot, year, day))
	if err != nil {
		ExitErr("Could not read input: %v\n", err)
	}

	return strings.TrimSuffix(string(bytes), "\n")
}
