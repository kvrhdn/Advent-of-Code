package day24

import "fmt"

type Component struct {
	id             int
	value1, value2 int
}

func parseComponent(id int, input string) (c Component) {
	c.id = id
	fmt.Sscanf(input, "%d/%d", &c.value1, &c.value2)
	return
}

func (c *Component) strength() int {
	return c.value1 + c.value2
}

func (c *Component) tryConnectTo(port int) (otherPort int, ok bool) {
	if c.value1 == port {
		return c.value2, true
	}
	if c.value2 == port {
		return c.value1, true
	}
	return 0, false
}
