package main

import (
	"fmt"
	"github.com/koenaad/Advent-of-Code-2017/day10/knotHash"
	"github.com/koenaad/Advent-of-Code-2017/util"
	"strings"
)

var input = "97,167,54,178,2,11,209,174,119,248,254,0,255,1,64,190"

func main() {
	fmt.Println("Advent of Code 2017 - day 10")

	lengths := util.SliceAtoi(strings.Split(input, ","))

	numbers := knotHash.CreateNumbersUpTo(255)

	knotHash.KnotHashRound(numbers, lengths, 0, 0)

	fmt.Printf("Solution 1: multiplication of first two numbers = %v\n", numbers[0]*numbers[1])

	denseKnotHash := knotHash.DenseKnotHash(input)

	fmt.Printf("Solution 2: dense knot hash = %v\n", denseKnotHash)
}
