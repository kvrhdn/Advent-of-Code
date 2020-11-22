package day13

import (
	"sort"
	"strings"
)

func SolvePart1(input string) interface{} {
	layers := parseInput(input)

	severityAccrued := 0

	for _, layer := range layers {
		if layer.isScannerAtZeroAfter(layer.depth) {
			severityAccrued += layer.depth * layer.scannerRange
		}
	}

	return severityAccrued
}

func SolvePart2(input string) interface{} {
	layers := parseInput(input)

	// sort by smallest range first, since these are most likely to catch you
	sort.Slice(layers, func(layer1, layer2 int) bool {
		return layers[layer1].scannerRange < layers[layer2].scannerRange
	})

	delay := 0
	for {
		if canPassWithoutBeingScanned(layers, delay) {
			return delay
		}
		delay++
	}
}

func parseInput(input string) (layers []Layer) {
	for _, line := range strings.Split(input, "\n") {
		layers = append(layers, parseLayer(line))
	}
	return
}

func canPassWithoutBeingScanned(layers []Layer, delay int) bool {
	for _, l := range layers {
		if l.isScannerAtZeroAfter(delay + l.depth) {
			return false
		}
	}
	return true
}
