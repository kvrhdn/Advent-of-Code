package duet

import (
	"fmt"
	"time"
)

type register rune

type controllerVersion uint8

const (
	v1 controllerVersion = iota
	v2
)

type Controller struct {
	id        int
	registers map[register]int
	irPointer int
	version   controllerVersion
	// v1
	LastFrequency int
	// v2
	sendChan   chan<- int
	recvChan   <-chan int
	doneChan   chan<- bool
	ValuesSent int
}

func NewControllerV1() (c Controller) {
	c.registers = make(map[register]int)
	c.version = v1
	return
}

func NewControllerV2(sendChan chan<- int, recvChan <-chan int, programId int) (c Controller, done <-chan bool) {
	c.id = programId
	c.registers = make(map[register]int)
	c.version = v2

	c.set('p', programId)

	c.sendChan = sendChan
	c.recvChan = recvChan

	doneChan := make(chan bool)
	c.doneChan = doneChan

	return c, doneChan
}

func (c *Controller) Run(instructions []Instruction) {
	for {
		ir := instructions[c.irPointer]

		fmt.Printf("%v | %v | %T %v\n", c.id, c.irPointer, ir, ir)

		shouldHalt := ir.operateOn(c)
		if shouldHalt {
			break
		}

		c.irPointer += 1
		if c.irPointer < 0 || c.irPointer >= len(instructions) {
			break
		}
	}

	if c.doneChan != nil {
		c.doneChan <- true
		close(c.doneChan)
	}
	fmt.Printf("%v | done\n", c.id)
}

func (c *Controller) get(r register) int {
	return c.registers[r]
}

func (c *Controller) set(r register, value int) {
	c.registers[r] = value
}

func (c *Controller) jump(offset int) {
	c.irPointer += offset - 1
}

func (c *Controller) playSound(frequency int) {
	if c.version != v1 {
		panic("instruction not supported")
	}

	c.LastFrequency = frequency
}

func (c *Controller) send(value int) (success bool) {
	if c.version != v2 {
		panic("instruction not supported")
	}

	select {
	case c.sendChan <- value:
		c.ValuesSent += 1
		success = true

	default:
		fmt.Printf("%v | can't send any more values\n", c.id)
		success = false
	}
	return
}

func (c *Controller) recv() (value int, timedOut bool) {
	if c.version != v2 {
		panic("instruction not supported")
	}

	select {
	case value = <-c.recvChan:
		timedOut = false

	case <-time.After(1 * time.Second):
		fmt.Printf("%v | timed out\n", c.id)
		timedOut = true
	}
	return
}
