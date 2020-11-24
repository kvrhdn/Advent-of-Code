package day24

type Bridge []Component

func (b *Bridge) strength() (strength int) {
	for _, c := range *b {
		strength += c.strength()
	}
	return
}

func (b *Bridge) length() int {
	return len(*b)
}

func buildAllPossibleBridges(usedComponents []Component, port int, allComponents []Component) (bridges []Bridge) {
scanAllComponents:
	for _, c := range allComponents {
		newPort, ok := c.tryConnectTo(port)
		if !ok {
			continue
		}

		for _, usedComponent := range usedComponents {
			if c.id == usedComponent.id {
				continue scanAllComponents
			}
		}

		// clone and append c
		newBridge := append([]Component(nil), usedComponents...)
		newBridge = append(newBridge, c)

		bridges = append(bridges, newBridge)
		bridges = append(bridges, buildAllPossibleBridges(newBridge, newPort, allComponents)...)
	}
	return
}
