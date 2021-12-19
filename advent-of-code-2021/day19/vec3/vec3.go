package vec3

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

type Vec3 struct {
	X, Y, Z int
}

func Parse(s string) (Vec3, error) {
	var values []int
	for _, valueStr := range strings.Split(s, ",") {
		value, err := strconv.Atoi(valueStr)
		if err != nil {
			return Vec3{}, err
		}
		values = append(values, value)
	}

	if len(values) != 3 {
		return Vec3{}, fmt.Errorf("expected 3 numbers, got %d", len(values))
	}

	return Vec3{values[0], values[1], values[2]}, nil
}

func (v Vec3) String() string {
	return fmt.Sprintf("%d,%d,%d", v.X, v.Y, v.Z)
}

func (v Vec3) Add(other Vec3) Vec3 {
	return Vec3{other.X + v.X, other.Y + v.Y, other.Z + v.Z}
}

func (v Vec3) Sub(other Vec3) Vec3 {
	return Vec3{other.X - v.X, other.Y - v.Y, other.Z - v.Z}
}

func (v Vec3) DistanceTo(other Vec3) int {
	return int(math.Abs(float64(other.X-v.X)) + math.Abs(float64(other.Y-v.Y)) + math.Abs(float64(other.Z-v.Z)))
}

// AllRotations returns 24 rotations of the Vec3 around (0,0,0).
func (v Vec3) AllRotations() []Vec3 {
	// copied from https://stackoverflow.com/a/16467849/10467175
	rotations := []Vec3{}

	roll := func(v Vec3) Vec3 {
		return Vec3{v.X, v.Z, -v.Y}
	}
	turn := func(v Vec3) Vec3 {
		return Vec3{-v.Y, v.X, v.Z}
	}

	times(2, func() {
		times(3, func() {
			v = roll(v)
			rotations = append(rotations, v)

			times(3, func() {
				v = turn(v)
				rotations = append(rotations, v)
			})
		})
		v = roll(turn(roll(v)))
	})

	return rotations
}

// AllRotations returns 24 []Vec3 rotated around (0,0,0).
func AllRotations(vecs []Vec3) [][]Vec3 {
	var rotatedVecs [][]Vec3
	times(24, func() {
		rotatedVecs = append(rotatedVecs, make([]Vec3, len(vecs)))
	})

	for elementIndex, v := range vecs {
		for rotationIndex, rotation := range v.AllRotations() {
			rotatedVecs[rotationIndex][elementIndex] = rotation
		}
	}

	return rotatedVecs
}
