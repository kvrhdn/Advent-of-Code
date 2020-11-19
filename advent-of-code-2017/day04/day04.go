package day04

import (
	"sort"
	"strings"
)

func SolvePart1(input string) interface{} {
	return countValidPassphrases(input, validatorNoDuplicates)
}

func SolvePart2(input string) interface{} {
	return countValidPassphrases(input, validatorNoAnagrams)
}

type Validator func(word1, word2 string) bool

func countValidPassphrases(input string, validator Validator) int {
	count := 0

	for _, passphrase := range strings.Split(input, "\n") {
		if validatePassphrase(passphrase, validator) {
			count++
		}
	}

	return count
}

func validatePassphrase(passphrase string, validator Validator) bool {
	words := strings.Fields(passphrase)

	for i, word1 := range words {
		for _, word2 := range words[i+1:] {
			if !validator(word1, word2) {
				return false
			}
		}
	}
	return true
}

func validatorNoDuplicates(word1, word2 string) bool {
	return word1 != word2
}

func validatorNoAnagrams(word1, word2 string) bool {
	if len(word1) != len(word2) {
		return true
	}

	runes1 := sortRunes([]rune(word1))
	runes2 := sortRunes([]rune(word2))

	for i := 0; i < len(runes1); i++ {
		if runes1[i] != runes2[i] {
			return true
		}
	}
	return false
}

func sortRunes(runes []rune) []rune {
	sort.Slice(runes, func(i, j int) bool {
		return runes[i] < runes[j]
	})
	return runes
}
