// Package aoc provides a command line interface to solve Advent of Code
// puzzles.
package aoc

import (
	"fmt"
	"os"
	"sort"

	"github.com/jessevdk/go-flags"
)

type Configuration struct {
	Year int
	Days map[int]Day
}

var opts struct {
	Day  *int `short:"d" long:"day" description:"The day to run, defaults to the latest."`
	Part *int `short:"p" long:"part" description:"The part to run, runs both parts if not specified."`
	All  bool `long:"all" description:"Run all days sequentially."`
}

func (c *Configuration) Run() {
	_, err := flags.NewParser(&opts, flags.HelpFlag).Parse()
	if err != nil {
		if flagsErr, ok := err.(*flags.Error); ok && flagsErr.Type == flags.ErrHelp {
			ExitMsg(flagsErr.Message)
		} else {
			ExitErr("Could not parse arguments: %v\n", err)
		}
	}

	fmt.Printf("Advent of Code %d\n\n", c.Year)

	if len(c.Days) == 0 {
		ExitErr("No solutions have been implemented yet.\n")
	}

	if opts.All {
		c.runAllDays()
		Exit()
	}

	if opts.Day != nil {
		c.runSpecificDay(*opts.Day, opts.Part)
		Exit()
	}

	latest := c.findLatestDay()
	c.runSpecificDay(latest, opts.Part)
	Exit()
}

func (c *Configuration) runAllDays() {
	var days []int

	for k, _ := range c.Days {
		days = append(days, k)
	}

	sort.Ints(days)

	for _, d := range days {
		day := c.Days[d]
		day.run(c.Year, d, nil)
	}
}

func (c *Configuration) findLatestDay() (latest int) {
	for k, _ := range c.Days {
		if k > latest {
			latest = k
		}
	}
	return latest
}

func (c *Configuration) runSpecificDay(day int, part *int) {
	d, ok := c.Days[day]
	if !ok {
		ExitErr("No solutions found for day %d.\n", day)
	}
	d.run(c.Year, day, part)
}

func Exit() {
	os.Exit(0)
}

func ExitMsg(format string, a ...interface{}) {
	fmt.Printf(format, a...)
	os.Exit(0)
}

func ExitErr(format string, a ...interface{}) {
	_, _ = os.Stderr.WriteString(fmt.Sprintf(format, a...))
	os.Exit(1)
}
