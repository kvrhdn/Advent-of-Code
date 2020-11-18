package grid

import (
	"fmt"
	"testing"
)

func TestInitGridKnotHash(t *testing.T) {
	expectedSquaresOccupied := 8108
	expectedGridPartly := [8][8]int{
		{1, 1, 0, 1, 0, 1, 0, 0},
		{0, 1, 0, 1, 0, 1, 0, 1},
		{0, 0, 0, 0, 1, 0, 1, 0},
		{1, 0, 1, 0, 1, 1, 0, 1},
		{0, 1, 1, 0, 1, 0, 0, 0},
		{1, 1, 0, 0, 1, 0, 0, 1},
		{0, 1, 0, 0, 0, 1, 0, 0},
		{1, 1, 0, 1, 0, 1, 1, 0},
	}

	grid := InitGridKnotHash("flqrgnkx")

	got := grid.SquaresOccupied()
	if got != expectedSquaresOccupied {
		t.Errorf("SquaresOccupied returned %v, but expected %v", got, expectedSquaresOccupied)
	}

	for i := range expectedGridPartly {
		for j := range expectedGridPartly[i] {
			expectedValue := expectedGridPartly[i][j]
			gotValue := grid.d[i][j]

			if expectedValue != gotValue {
				t.Errorf("Grid position %v, %v return %v, but expected %v", i, j, gotValue, expectedValue)
			}
		}
	}

	for _, r := range grid.d {
		for _, sq := range r {
			if sq > 0 {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
}

func TestGrid_Regionalize(t *testing.T) {
	expectedRegions := 1242

	//      01234567
	//
	// 0    11.2.3..-->
	// 1    .1.2.3.4
	// 2    ....5.6.
	// 3    7.8.55.9
	// 4    .88.5...
	// 5    88..5..8
	// 6    .8...8..
	// 7    88.8.88.-->
	//      |      |
	//      V      V
	expectedRegionsPartly := [][]location{
		{{0, 0}, {0, 1}, {1, 1}},                 // 1
		{{0, 3}, {1, 3}},                         // 2
		{{0, 5}, {1, 5}},                         // 3
		{{1, 7}},                                 // 4
		{{2, 4}, {3, 4}, {3, 5}, {4, 4}, {5, 4}}, // 5
		{{2, 6}},                                 // 6
		{{3, 0}},                                 // 7
		{{3, 2}, {4, 1}, {4, 2}, {5, 0}, {5, 1}, {5, 7}, {6, 1}, {6, 5}, {7, 0}, {7, 1}, {7, 3}, {7, 5}, {7, 6}}, // 8
		{{3, 7}}, // 9
	}

	grid := InitGridKnotHash("flqrgnkx")
	gotRegions := grid.Regionalize()

	if gotRegions != expectedRegions {
		t.Errorf("Regions = %v, but expected %v", gotRegions, expectedRegions)
	}

	for _, expectedRegion := range expectedRegionsPartly {
		loc0 := expectedRegion[0]
		regionId := grid.getValue(loc0.row, loc0.col)

		for _, loc := range expectedRegion {
			if grid.getValue(loc.row, loc.col) != regionId {
				t.Errorf("Expected region to contain %v, but %v has a different region id", expectedRegion, loc)
			}
		}
	}
}

type location struct {
	row, col int
}

func (l *location) String() string {
	return fmt.Sprintf("[%v, %v]", l.row, l.col)
}
