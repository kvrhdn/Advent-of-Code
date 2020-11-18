package day15

import (
	"testing"
)

func TestCreateGenerator(t *testing.T) {
	cases := []struct {
		generator      GeneratorDefinition
		expectedValues []uint64
	}{
		{GeneratorDefinition{65, 16807, nil}, []uint64{1092455, 1181022009, 245556042, 1744312007, 1352636452}},
		{GeneratorDefinition{8921, 48271, nil}, []uint64{430625591, 1233683848, 1431495498, 137874439, 285222916}},
		{GeneratorDefinition{65, 16807, notDivisbleBy(4)}, []uint64{1352636452, 1992081072, 530830436, 1980017072, 740335192}},
		{GeneratorDefinition{8921, 48271, notDivisbleBy(8)}, []uint64{1233683848, 862516352, 1159784568, 1616057672, 412269392}},
	}

	for cNr, c := range cases {
		dataChan, stopChan := CreateGenerator(c.generator)

		for i, expected := range c.expectedValues {
			got := <-dataChan
			if got != expected {
				t.Errorf("Generator %v value %v: got %v but expected %v", cNr, i, got, expected)
			}
		}

		close(stopChan)
	}
}

func TestJudge(t *testing.T) {
	cases := []struct {
		genDefA, genDefB GeneratorDefinition
		count            int
		expected         int
	}{
		{GeneratorDefinition{65, 16807, nil}, GeneratorDefinition{8921, 48271, nil}, 5, 1},
		{GeneratorDefinition{65, 16807, nil}, GeneratorDefinition{8921, 48271, nil}, 40000000, 588},
		{GeneratorDefinition{65, 16807, notDivisbleBy(4)}, GeneratorDefinition{8921, 48271, notDivisbleBy(8)}, 5, 0},
		{GeneratorDefinition{65, 16807, notDivisbleBy(4)}, GeneratorDefinition{8921, 48271, notDivisbleBy(8)}, 5000000, 309},
	}

	for _, c := range cases {
		generatorA, stopChanA := CreateGenerator(c.genDefA)
		generatorB, stopChanB := CreateGenerator(c.genDefB)

		got := Judge(generatorA, generatorB, c.count)
		if got != c.expected {
			t.Errorf("Judge return %v, but expected %v", got, c.expected)
		}

		close(stopChanA)
		close(stopChanB)
	}
}
