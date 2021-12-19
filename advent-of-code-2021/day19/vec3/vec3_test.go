package vec3

import (
	"testing"

	"github.com/stretchr/testify/assert"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/shared/set"
)

func TestParse(t *testing.T) {
	testCases := []string{
		"404,-588,-901",
		"528,-643,409",
		"-838,591,734",
		"390,-675,-793",
		"-537,-823,-458",
		"-485,-357,347",
		"-345,-311,381",
		"-661,-816,-575",
		"-876,649,763",
		"-618,-824,-621",
		"553,345,-567",
		"474,580,667",
		"-447,-329,318",
		"-584,868,-557",
		"544,-627,-890",
		"564,392,-477",
		"455,729,728",
		"-892,524,684",
		"-689,845,-530",
		"423,-701,434",
		"7,-33,-71",
		"630,319,-379",
		"443,580,662",
		"-789,900,-551",
		"459,-707,401",
	}
	for _, testCase := range testCases {
		value, err := Parse(testCase)
		assert.NoError(t, err)
		assert.Equal(t, testCase, value.String())
	}
}

func TestVec3_AllRotations(t *testing.T) {
	v := Vec3{1, 2, 3}
	rotations := v.AllRotations()

	assert.Len(t, rotations, 24)

	// verify the rotations are all unique
	set := set.New[Vec3]()
	for _, r := range rotations {
		set.Add(r)
	}
	assert.Equal(t, 24, set.Len())
}

func TestRotateAll(t *testing.T) {
	original := []Vec3{
		{-1, -1, 1},
		{-2, -2, 2},
		{-3, -3, 3},
		{-2, -3, 1},
		{5, 6, -4},
		{8, 0, 7},
	}

	exampleRotations := [][]Vec3{
		original,
		{
			{1, -1, 1},
			{2, -2, 2},
			{3, -3, 3},
			{2, -1, 3},
			{-5, 4, -6},
			{-8, -7, 0},
		},
		{
			{-1, -1, -1},
			{-2, -2, -2},
			{-3, -3, -3},
			{-1, -3, -2},
			{4, 6, 5},
			{-7, 0, 8},
		},
		{
			{1, 1, -1},
			{2, 2, -2},
			{3, 3, -3},
			{1, 3, -2},
			{-4, -6, 5},
			{7, 0, 8},
		},
		{
			{1, 1, 1},
			{2, 2, 2},
			{3, 3, 3},
			{3, 1, 2},
			{-6, -4, -5},
			{0, 7, -8},
		},
	}

	allRotations := AllRotations(original)
	for _, exampleRotation := range exampleRotations {
		assert.Contains(t, allRotations, exampleRotation)
	}
}
