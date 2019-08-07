package main

import (
	"fmt"
	"testing"
)

func Test_puzzle1(t *testing.T) {
	expected := 8108

	grid := initGrid("flqrgnkx")

	got := grid.SquaresUsed()
	if got != expected {
		t.Errorf("SquaresUsed returned %v, but expected %v", got, expected)
	}

	for _, r := range grid.d {
		for _, sq := range r {
			if sq {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
}
