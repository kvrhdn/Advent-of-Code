package main

import (
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day01"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day02"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day03"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day04"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day05"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day06"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day07"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day08"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day09"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day10"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day11"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day12"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day13"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day14"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day15"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day16"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day17"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day18"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day19"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day20"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day21"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day22"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day23"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day24"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day25"
)

func main() {
	config := aoc.Configuration{
		Year: 2017,
		Days: map[int]aoc.Day{
			1: {
				Part1: day01.SolvePart1,
				Part2: day01.SolvePart2,
			},
			2: {
				Part1: day02.SolvePart1,
				Part2: day02.SolvePart2,
			},
			3: {
				Part1: day03.SolvePart1,
				Part2: day03.SolvePart2,
			},
			4: {
				Part1: day04.SolvePart1,
				Part2: day04.SolvePart2,
			},
			5: {
				Part1: day05.SolvePart1,
				Part2: day05.SolvePart2,
			},
			6: {
				Part1: day06.SolvePart1,
				Part2: day06.SolvePart2,
			},
			7: {
				Part1: day07.SolvePart1,
				Part2: day07.SolvePart2,
			},
			8: {
				Part1: day08.SolvePart1,
				Part2: day08.SolvePart2,
			},
			9: {
				Part1: day09.SolvePart1,
				Part2: day09.SolvePart2,
			},
			10: {
				Part1: day10.SolvePart1,
				Part2: day10.SolvePart2,
			},
			11: {
				Part1: day11.SolvePart1,
				Part2: day11.SolvePart2,
			},
			12: {
				Part1: day12.SolvePart1,
				Part2: day12.SolvePart2,
			},
			13: {
				Part1: day13.SolvePart1,
				Part2: day13.SolvePart2,
			},
			14: {
				Part1: day14.SolvePart1,
				Part2: day14.SolvePart2,
			},
			15: {
				Part1: day15.SolvePart1,
				Part2: day15.SolvePart2,
			},
			16: {
				Part1: day16.SolvePart1,
				Part2: day16.SolvePart2,
			},
			17: {
				Part1: day17.SolvePart1,
				Part2: day17.SolvePart2,
			},
			18: {
				Part1: day18.SolvePart1,
				Part2: day18.SolvePart2,
			},
			19: {
				Part1: day19.SolvePart1,
				Part2: day19.SolvePart2,
			},
			20: {
				Part1: day20.SolvePart1,
				Part2: day20.SolvePart2,
			},
			21: {
				Part1: day21.SolvePart1,
				Part2: day21.SolvePart2,
			},
			22: {
				Part1: day22.SolvePart1,
				Part2: day22.SolvePart2,
			},
			23: {
				Part1: day23.SolvePart1,
				Part2: day23.SolvePart2,
			},
			24: {
				Part1: day24.SolvePart1,
				Part2: day24.SolvePart2,
			},
			25: {
				Part1: day25.SolvePart1,
			},
		},
	}
	config.Run()
}
