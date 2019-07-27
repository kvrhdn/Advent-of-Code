package util

import "strconv"

func SliceEquals(slice1, slice2 []int) bool {
	if len(slice1) != len(slice2) {
		return false
	}

	for i := 0; i < len(slice1); i++ {
		if slice1[i] != slice2[i] {
			return false
		}
	}

	return true
}

func SliceCopy(in []int) []int {
	out := make([]int, len(in))

	copy(out, in)

	return out
}

func SliceAtoi(in []string) []int {
	var ret = make([]int, len(in))

	for i, a := range in {
		value, err := strconv.Atoi(a)

		if err != nil {
			panic(err)
		}

		ret[i] = value
	}

	return ret
}
