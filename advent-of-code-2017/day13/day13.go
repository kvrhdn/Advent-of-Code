package day13

import (
	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day13/firewall"
)

func SolvePart1(input string) interface{} {
	f := firewall.Init(input)

	return f.SeverityAccruedAfterSteppingThrough()
}

func SolvePart2(input string) interface{} {
	f := firewall.Init(input)

	return delayNeededToAvoidGettingCaught(&f)
}

func delayNeededToAvoidGettingCaught(f *firewall.Firewall) int {
	delay := 0

	for {
		if f.CanStepThroughWithoutGettingCaughtAfter(delay) {
			return delay
		}
		delay += 1
	}
}
