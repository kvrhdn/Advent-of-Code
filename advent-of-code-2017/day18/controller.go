package day18

import "time"

type Register rune

type ControllerVersion int

const (
	v1 ControllerVersion = iota
	v2
)

type Controller struct {
	registers map[Register]int
	irPointer int
	version   ControllerVersion

	// v1
	lastFrequency int

	// v2
	sendChan   chan<- int
	recvChan   <-chan int
	doneChan   chan<- bool
	valuesSent int
}

func newControllerV1() (c Controller) {
	c.registers = make(map[Register]int)
	c.version = v1
	return
}

func newControllerV2(sendChan chan<- int, recvChan <-chan int, programID int) (c Controller, done <-chan bool) {
	c.registers = make(map[Register]int)
	c.version = v2

	c.set('p', programID)

	c.sendChan = sendChan
	c.recvChan = recvChan

	doneChan := make(chan bool)
	c.doneChan = doneChan

	return c, doneChan
}

func (c *Controller) execute(instructions []Instruction) {
	for {
		halt := instructions[c.irPointer].apply(c)
		if halt {
			break
		}

		c.irPointer++
	}

	// v2 only
	if c.doneChan != nil {
		c.doneChan <- true
		close(c.doneChan)
	}
}

func (c *Controller) get(r Register) int {
	return c.registers[r]
}

func (c *Controller) set(r Register, value int) {
	c.registers[r] = value
}

func (c *Controller) jump(offset int) {
	c.irPointer += offset - 1
}

func (c *Controller) playSound(frequency int) {
	if c.version != v1 {
		panic("instruction not supported")
	}

	c.lastFrequency = frequency
}

func (c *Controller) send(value int) (success bool) {
	if c.version != v2 {
		panic("instruction not supported")
	}

	select {
	case c.sendChan <- value:
		c.valuesSent++
		success = true

	default:
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

		// block for just an instant to yield to other controller
	case <-time.After(1 * time.Millisecond):
		timedOut = true
	}
	return
}
