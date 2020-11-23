package day18

import "strings"

func SolvePart1(input string) interface{} {
	instructions := parseInput(input)

	c := newControllerV1()
	c.execute(instructions)
	return c.lastFrequency
}

func SolvePart2(input string) interface{} {
	instructions := parseInput(input)

	chanSize := 200

	chan1 := make(chan int, chanSize)
	defer close(chan1)

	chan2 := make(chan int, chanSize)
	defer close(chan2)

	c0, done0 := newControllerV2(chan1, chan2, 0)
	go c0.execute(instructions)

	c1, done1 := newControllerV2(chan2, chan1, 1)
	go c1.execute(instructions)

	// block until controller 0 and 1 is done
	<-done0
	<-done1

	return c1.valuesSent
}

func parseInput(in string) (irs []Instruction) {
	for _, line := range strings.Split(in, "\n") {
		irs = append(irs, parseInstruction(line))
	}
	return irs
}
