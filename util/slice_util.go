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

func SliceCopy(in []int) (out []int) {
	return append(in[:0:0], in...)
}

func SliceAtoi(in []string) []int {
	var ret = make([]int, 0, len(in))

	for _, a := range in {
		i, err := strconv.Atoi(a)

		if err != nil {
			panic(err)
		}

		ret = append(ret, i)
	}

	return ret
}