package day13

import (
	"testing"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/day13/firewall"
)

func Test_delayNeedToAvoidBeingCaught(t *testing.T) {
	exampleInput := `0: 3
1: 2
4: 4
6: 4`

	expected := 10

	f := firewall.Init(exampleInput)

	got := delayNeededToAvoidGettingCaught(&f)
	if got != expected {
		t.Errorf("delayNeededToAvoidGettingCaught: got %v, but expected %v", got, expected)
	}
}
