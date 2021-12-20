package day20

import (
	"context"
	"math"
	"strconv"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
)

var Solution = aoc.NewDayGen(generator, part1, part2)

type vec2 struct {
	x, y int
}

type imageEnhancementAlgorithm struct {
	value string
}

func (i *imageEnhancementAlgorithm) Lookup(index int) bool {
	if index >= 512 {
		panic("can not lookup index greater than or equal to 512")
	}
	return i.value[index] == '#'
}

type image struct {
	grid map[vec2]bool
}

func parseImage(input string) image {
	i := image{grid: make(map[vec2]bool)}

	for y, line := range strings.Split(input, "\n") {
		for x, r := range line {
			if r == '#' {
				i.grid[vec2{x, y}] = true
			}
		}
	}

	return i
}

func (i image) minMax() (minX, maxX, minY, maxY int) {
	minX = math.MaxInt
	maxX = math.MinInt
	minY = math.MaxInt
	maxY = math.MinInt

	for k, _ := range i.grid {
		if k.x < minX {
			minX = k.x
		}
		if k.x > maxX {
			maxX = k.x
		}
		if k.y < minY {
			minY = k.y
		}
		if k.y > maxY {
			maxY = k.y
		}
	}

	return minX, maxX, minY, maxY
}

func (i image) String() string {
	minX, maxX, minY, maxY := i.minMax()

	var s strings.Builder

	for y := minY; y <= maxY; y++ {
		for x := minX; x <= maxX; x++ {
			if i.grid[vec2{x, y}] {
				s.WriteRune('#')
			} else {
				s.WriteRune('.')
			}
		}
		s.WriteRune('\n')
	}

	return s.String()
}

func (i image) Enhanced(algorithm imageEnhancementAlgorithm) image {
	minX, maxX, minY, maxY := i.minMax()

	new := image{grid: make(map[vec2]bool)}

	// add a good amount of overscan:
	//  - this is necessary to compensate for the growing image
	//  - this will create a chunky border if 0 is unstable
	overscan := 16

	for y := minY - overscan; y <= maxY+overscan; y++ {
		for x := minX - overscan; x <= maxX+overscan; x++ {
			// calculate lookup index
			var binary strings.Builder
			for y2 := y - 1; y2 <= y+1; y2++ {
				for x2 := x - 1; x2 <= x+1; x2++ {
					if i.grid[vec2{x2, y2}] {
						binary.WriteRune('1')
					} else {
						binary.WriteRune('0')
					}
				}
			}
			index, _ := strconv.ParseInt(binary.String(), 2, 32)

			// set new pixel
			if algorithm.Lookup(int(index)) {
				new.grid[vec2{x, y}] = true
			}
		}
	}

	return new
}

// TrimmedBorder creates a new image with the border removed.
func (i image) TrimmedBorder() image {
	minX, maxX, minY, maxY := i.minMax()

	new := image{grid: make(map[vec2]bool)}

	// assumption: a border will show up as a band of pixels with the thickness size on every side
	borderThickness := 0
	// iterate from left to right until we find a not lit pixel (= the border has stopped)
	middleY := (minY + maxY) / 2
	for x := minX; x < maxX; x++ {
		if !i.grid[(vec2{x, middleY})] {
			borderThickness = x - minX
			break
		}
	}

	for y := minY + borderThickness; y <= maxY-borderThickness; y++ {
		for x := minX + borderThickness; x <= maxX-borderThickness; x++ {
			v := vec2{x, y}
			if i.grid[v] {
				new.grid[v] = true
			}
		}
	}

	return new
}

func (i image) LitPixels() int {
	litPixels := 0
	for _, pixel := range i.grid {
		if pixel {
			litPixels += 1
		}
	}
	return litPixels
}

type processedInput struct {
	imageEnhancementAlgorithm imageEnhancementAlgorithm
	image                     image
}

func generator(ctx context.Context, input string) (processedInput, error) {
	paragraphs := strings.Split(input, "\n\n")

	return processedInput{
		imageEnhancementAlgorithm: imageEnhancementAlgorithm{value: paragraphs[0]},
		image:                     parseImage(paragraphs[1]),
	}, nil
}

func part1(ctx context.Context, input processedInput) (interface{}, error) {
	i := input.image
	//fmt.Println(i)

	for times := 0; times < 2; times++ {
		i = i.Enhanced(input.imageEnhancementAlgorithm)

		//fmt.Println("Iteration", times)
		//fmt.Println()
		//fmt.Println(i)
	}

	// trim the border if 0 is unstable
	if input.imageEnhancementAlgorithm.Lookup(0) {
		i = i.TrimmedBorder()
	}

	//fmt.Println("After trim")
	//fmt.Println()
	//fmt.Println(i)

	return i.LitPixels(), nil
}

func part2(ctx context.Context, input processedInput) (interface{}, error) {
	i := input.image
	//fmt.Println(i)

	for times := 0; times < 50; times++ {
		i = i.Enhanced(input.imageEnhancementAlgorithm)

		// 0 is unstable
		if input.imageEnhancementAlgorithm.Lookup(0) && times%2 == 1 {
			//fmt.Println("Iteration", times, "before trim")
			//fmt.Println()
			//fmt.Println(i)

			i = i.TrimmedBorder()
		}

		//fmt.Println("Iteration", times)
		//fmt.Println()
		//fmt.Println(i)
	}

	return i.LitPixels(), nil
}
