package main

import (
	"reflect"
	"testing"
)

func TestFollow(t *testing.T) {
	expectedPath := []rune("ABCDEF")
	expectedSteps := 38

	input := `
     |
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+`

	rd := NewRoutingDiagram(input)

	gotPath, gotSteps := StepThrough(rd)
	if !reflect.DeepEqual(gotPath, expectedPath) || gotSteps != expectedSteps {
		t.Errorf("Followed routing diagram got %q, %v, but expected %q, %v", gotPath, gotSteps, expectedPath, expectedSteps)
	}
}
