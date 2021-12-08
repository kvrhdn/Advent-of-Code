package strings

import (
	"sort"
	"strings"
)

func Sort(s string) string {
	runes := []rune(s)
	sort.Slice(runes, func(i, j int) bool {
		return runes[i] < runes[j]
	})
	return string(runes)
}

func RemoveRunes(s, toRemove string) string {
	return strings.Map(func(r rune) rune {
		if strings.ContainsAny(string(r), toRemove) {
			return -1
		} else {
			return r
		}
	}, s)
}

func KeepRunes(s, toKeep string) string {
	return strings.Map(func(r rune) rune {
		if strings.ContainsAny(string(r), toKeep) {
			return r
		} else {
			return -1
		}
	}, s)
}
