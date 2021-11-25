package aoc

import (
	"context"
	"flag"
	"fmt"
	"sort"
)

const (
	year = 2021
)

type Configuration struct {
	Days map[int]Day
}

func (c *Configuration) Run(ctx context.Context) error {
	day := flag.Int("d", 0, "The day to run, runs all if omitted")
	part := flag.Int("p", 0, "The part to run (1 or 2), runs both if omitted")
	flag.Parse()

	fmt.Printf("Advent of Code %d\n\n", year)

	if *day == 0 {
		for _, day := range c.daysSorted() {
			solution := c.Days[day]
			err := solution.run(ctx, day, *part)
			if err != nil {
				return err
			}
		}
		return nil
	}

	solution, ok := c.Days[*day]
	if !ok {
		return fmt.Errorf("could not find solution for day %d", *day)
	}

	return solution.run(ctx, *day, *part)
}

func (c *Configuration) daysSorted() (days []int) {
	for day := range c.Days {
		days = append(days, day)
	}
	sort.Ints(days)
	return
}
