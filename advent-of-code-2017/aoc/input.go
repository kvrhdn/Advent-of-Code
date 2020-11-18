package aoc

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func readInput(year, day int) string {
	bytes, err := ioutil.ReadFile(fmt.Sprintf("input/%02d/day%02d.txt", year, day))
	if err != nil {
		ExitErr("Could not read input: %v\n", err)
	}

	return strings.TrimSuffix(string(bytes), "\n")
}
