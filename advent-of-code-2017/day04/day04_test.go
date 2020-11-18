package main

import (
	"testing"
)

func TestValidatePassphraseNoDuplicates(t *testing.T) {
	cases := []struct {
		in       string
		expected bool
	}{
		{"aa bb cc dd ee", true},
		{"aa bb cc dd aa", false},
		{"aa bb cc dd aaa", true},
	}
	for _, c := range cases {
		got := ValidatePassphrase(c.in, NoDuplicates)
		if got != c.expected {
			t.Errorf("ValidatePassphrase(%v, NoDuplicates) = %v, but expected %v", c.in, got, c.expected)
		}
	}
}

func TestValidatePassphraseNoAnagrams(t *testing.T) {
	cases := []struct {
		in       string
		expected bool
	}{
		{"abcde fghij", true},
		{"abcde xyz ecdab", false},
		{"a ab abc abd abf abj", true},
		{"iiii oiii ooii oooi oooo", true},
		{"oiii ioii iioi iiio", false},
	}
	for _, c := range cases {
		got := ValidatePassphrase(c.in, NoAnagrams)
		if got != c.expected {
			t.Errorf("ValidatePassphrase(%v, NoAnagrams) = %v, but expected %v", c.in, got, c.expected)
		}
	}
}

