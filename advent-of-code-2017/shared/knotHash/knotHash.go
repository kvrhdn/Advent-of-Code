package knotHash

import (
	"fmt"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2017/shared/util"
)

func DenseKnotHash(input string) string {
	knot := newDenseKnotHash(input)

	util.Times(64, func() {
		knot.DoARound()
	})

	return knot.denseHash()
}

type KnotHash struct {
	Numbers  []int
	lengths  []int
	position int
	skipSize int
}

func New(lengths []int) *KnotHash {
	return newWithCustomSize(256, lengths)
}

func newWithCustomSize(size int, lengths []int) *KnotHash {
	return &KnotHash{
		Numbers:  createSequence(size),
		lengths:  lengths,
		position: 0,
		skipSize: 0,
	}
}

func newDenseKnotHash(input string) *KnotHash {
	lengths := make([]int, len(input)+5)

	for i, c := range []rune(input) {
		lengths[i] = int(c)
	}

	offset := len(input)
	for i, c := range []int{17, 31, 73, 47, 23} {
		lengths[offset+i] = c
	}

	return New(lengths)
}

func (k *KnotHash) wrapIndex(index int) int {
	if index < 0 {
		index += len(k.Numbers)
	}
	return index % len(k.Numbers)
}

func (k *KnotHash) DoARound() {
	for _, length := range k.lengths {
		from := k.wrapIndex(k.position)
		to := k.wrapIndex(k.position + length - 1)

		util.Times(length/2, func() {
			k.Numbers[from], k.Numbers[to] = k.Numbers[to], k.Numbers[from]

			from = k.wrapIndex(from + 1)
			to = k.wrapIndex(to - 1)
		})

		k.position += length + k.skipSize
		k.skipSize += 1
	}
}

func (k *KnotHash) denseHash() string {
	if len(k.Numbers)%16 != 0 {
		panic("numbers should be multiple of 16")
	}

	sparseHash := k.Numbers
	denseHash := make([]int, len(sparseHash)/16)

	for i := 0; i < len(k.Numbers)/16; i++ {
		offset := i * 16

		acc := 0
		for _, n := range k.Numbers[offset : offset+16] {
			acc ^= n
		}

		denseHash[i] = acc
	}

	var hexDenseHash strings.Builder

	for _, v := range denseHash {
		hexDenseHash.WriteString(fmt.Sprintf("%02x", v))
	}

	return hexDenseHash.String()
}
