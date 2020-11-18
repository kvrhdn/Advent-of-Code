package day18

import (
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day18/duet"
)

func SolvePart1(input string) interface{} {
	instructions, err := duet.ParseInstructions(input)
	if err != nil {
		panic(err)
	}

	return findLastPlayedFrequency(instructions)
}

func SolvePart2(input string) interface{} {
	instructions, err := duet.ParseInstructions(input)
	if err != nil {
		panic(err)
	}

	return valuesSentByProgram1(instructions)
}

func findLastPlayedFrequency(instructions []duet.Instruction) int {
	c := duet.NewControllerV1()
	c.Run(instructions)
	return c.LastFrequency
}

func valuesSentByProgram1(instructions []duet.Instruction) int {
	chanSize := 1000

	chan1 := make(chan int, chanSize)
	defer close(chan1)

	chan2 := make(chan int, chanSize)
	defer close(chan2)

	c0, done0 := duet.NewControllerV2(chan1, chan2, 0)
	go c0.Run(instructions)

	c1, done1 := duet.NewControllerV2(chan2, chan1, 1)
	go c1.Run(instructions)

	// block until controller 0 and 1 is done
	<-done0
	<-done1

	return c1.ValuesSent
}

const input = `set i 31
set a 1
mul p 17
jgz p p
mul a 2
add i -1
jgz i -2
add a -1
set i 127
set p 622
mul p 8505
mod p a
mul p 129749
add p 12345
mod p a
set b p
mod b 10000
snd b
add i -1
jgz i -9
jgz a 3
rcv b
jgz b -1
set f 0
set i 126
rcv a
rcv b
set p a
mul p -1
add p b
jgz p 4
snd a
set a b
jgz 1 3
snd b
set f 1
add i -1
jgz i -11
snd a
jgz f -16
jgz a -19`
