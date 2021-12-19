package day19

import (
	"context"
	"fmt"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/day19/vec3"
	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/set"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

type scanner struct {
	beacons []vec3.Vec3
}

func parseScanners(input string) ([]scanner, error) {
	var scanners []scanner

	for _, paragraph := range strings.Split(input, "\n\n") {
		var s scanner

		for i, line := range strings.Split(paragraph, "\n") {
			if i == 0 {
				continue // discard the first line
			}

			value, err := vec3.Parse(line)
			if err != nil {
				return nil, err
			}
			s.beacons = append(s.beacons, value)
		}

		scanners = append(scanners, s)
	}

	return scanners, nil
}

type processedInput struct {
	beacons  set.Set[vec3.Vec3]
	scanners []vec3.Vec3
}

func generator(ctx context.Context, input string) (processedInput, error) {
	scanners, err := parseScanners(input)
	if err != nil {
		return processedInput{}, err
	}

	beacons := set.New[vec3.Vec3]()
	var scannerPositions []vec3.Vec3

	// assume scanner 0 is at 0,0,0 and add all beacons
	for _, beacon := range scanners[0].beacons {
		beacons.Add(beacon)
	}
	scanners = scanners[1:]
	scannerPositions = append(scannerPositions, vec3.Vec3{0, 0, 0})

	// loop until all scanners have been processed
	for len(scanners) > 0 {
		var scannersRemaining []scanner

	testingScanners:
		for _, s := range scanners {

			// try all rotations of this scanner
			for _, rotation := range vec3.AllRotations(s.beacons) {
				// calculate differences between all known and new beacons
				diffs := make(map[vec3.Vec3]int)

				for _, knownBeacon := range beacons.Values() {
					for _, newBeacon := range rotation {
						diff := newBeacon.Sub(knownBeacon)
						diffs[diff] += 1
					}
				}

				// a diff can indicate a valid scanner position if at least 12 points share this diff
				var potentialPositions []vec3.Vec3
				for diff, count := range diffs {
					if count >= 12 {
						potentialPositions = append(potentialPositions, diff)
					}
				}

				if len(potentialPositions) == 0 {
					// try the next rotation
					continue
				}
				if len(potentialPositions) > 1 {
					panic(fmt.Sprintf("found multiple potential positions: %v", diffs))
				}

				// found the position of this scanner!
				position := potentialPositions[0]
				for _, newBeacon := range rotation {
					beacons.Add(newBeacon.Add(position))
				}
				scannerPositions = append(scannerPositions, position)

				continue testingScanners
			}

			// didn't find a match
			scannersRemaining = append(scannersRemaining, s)
		}

		scanners = scannersRemaining
	}

	return processedInput{
		beacons:  beacons,
		scanners: scannerPositions,
	}, nil
}

func part1(ctx context.Context, input processedInput) (interface{}, error) {
	return input.beacons.Len(), nil
}

func part2(ctx context.Context, input processedInput) (interface{}, error) {
	largestDistance := 0

	for _, p1 := range input.scanners {
		for _, p2 := range input.scanners {
			distance := p1.DistanceTo(p2)
			if distance > largestDistance {
				largestDistance = distance
			}
		}
	}

	return largestDistance, nil
}
