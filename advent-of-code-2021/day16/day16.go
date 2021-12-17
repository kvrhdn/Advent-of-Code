package day16

import (
	"context"
	"fmt"
	"math"
	"strings"

	"github.com/kvrhdn/advent-of-code/advent-of-code-2021/aoc"
)

var Solution = aoc.NewDay(part1, part2)

const (
	// type ID
	typeSum         = 0
	typeProduct     = 1
	typeMinimum     = 2
	typeMaximum     = 3
	typeLiteral     = 4
	typeGreaterThan = 5
	typeLessThan    = 6
	tpyeEqualTo     = 7

	lengthTypeTotalLength   = 0
	lengthTypeNumberPackets = 1
)

type packet struct {
	// header
	version int // 3 bits
	typeID  int // 3 bits

	// literal
	literal int64 // dynamic

	// operator
	lengthTypeID    int // 1 bit
	totalLength     int // 15 bits
	numberOfPackets int // 11 bits

	subPackets []packet // dynamic
}

func (p *packet) Print(indent string) {
	fmt.Printf("%sversion: %d\n", indent, p.version)
	fmt.Printf("%stype ID: %d\n", indent, p.typeID)

	if p.typeID == typeLiteral {
		fmt.Printf("%sliteral: %d\n", indent, p.literal)
	} else {
		fmt.Printf("%slength type ID: %d\n", indent, p.lengthTypeID)
		if p.lengthTypeID == lengthTypeTotalLength {
			fmt.Printf("%stotal length:   %d\n", indent, p.totalLength)
		} else {
			fmt.Printf("%snumber of packets: %d\n", indent, p.numberOfPackets)
		}
		for i, subPacket := range p.subPackets {
			fmt.Printf("%s%d:\n", indent, i)
			subPacket.Print(indent + "  ")
		}
	}
}

func decodePackets(ctx context.Context, binary string) (p packet, n int) {
	readStream := func(count int) string {
		s := binary[n : n+count]
		n += count
		return s
	}

	// read header

	p.version = parseBinary(readStream(3))
	p.typeID = parseBinary(readStream(3))

	// read payload

	// literal
	if p.typeID == typeLiteral {
		var literal strings.Builder
		for {
			group := readStream(5)

			literal.WriteString(group[1:5])
			if group[0] == '0' {
				break
			}
		}
		p.literal = parseBinaryInt64(literal.String())
		return
	}

	// operator
	p.lengthTypeID = parseBinary(readStream(1))

	if p.lengthTypeID == lengthTypeTotalLength {
		p.totalLength = parseBinary(readStream(15))

		totalConsumed := 0
		for totalConsumed < p.totalLength {
			subPacket, consumed := decodePackets(ctx, binary[n:])

			readStream(consumed)
			totalConsumed += consumed

			p.subPackets = append(p.subPackets, subPacket)
		}
	} else {
		p.numberOfPackets = parseBinary(readStream(11))

		for i := 0; i < p.numberOfPackets; i++ {
			subPacket, consumed := decodePackets(ctx, binary[n:])
			readStream(consumed)

			p.subPackets = append(p.subPackets, subPacket)
		}
	}

	return
}

func (p *packet) VersionSum() int {
	sum := p.version
	for _, subPacket := range p.subPackets {
		sum += subPacket.VersionSum()
	}
	return sum
}

func (p *packet) CalculateValue() int64 {
	switch p.typeID {
	case typeSum:
		var sum int64 = 0
		for _, subPacket := range p.subPackets {
			sum += subPacket.CalculateValue()
		}
		return sum
	case typeProduct:
		var product int64 = 1
		for _, subPacket := range p.subPackets {
			product *= subPacket.CalculateValue()
		}
		return product
	case typeMinimum:
		var min int64 = math.MaxInt64
		for _, subPacket := range p.subPackets {
			value := subPacket.CalculateValue()
			if value < min {
				min = value
			}
		}
		return min
	case typeMaximum:
		var max int64 = math.MinInt
		for _, subPacket := range p.subPackets {
			value := subPacket.CalculateValue()
			if value > max {
				max = value
			}
		}
		return max
	case typeLiteral:
		return p.literal
	case typeGreaterThan:
		if p.subPackets[0].CalculateValue() > p.subPackets[1].CalculateValue() {
			return 1
		} else {
			return 0
		}
	case typeLessThan:
		if p.subPackets[0].CalculateValue() < p.subPackets[1].CalculateValue() {
			return 1
		} else {
			return 0
		}
	case tpyeEqualTo:
		if p.subPackets[0].CalculateValue() == p.subPackets[1].CalculateValue() {
			return 1
		} else {
			return 0
		}
	}
	// unreachable
	return 0
}

func part1(ctx context.Context, input string) (interface{}, error) {
	binary := hexToBinary(input)
	p, _ := decodePackets(ctx, binary)
	//p.Print("")
	return p.VersionSum(), nil
}

func part2(ctx context.Context, input string) (interface{}, error) {
	binary := hexToBinary(input)
	p, _ := decodePackets(ctx, binary)
	//p.Print("")
	return p.CalculateValue(), nil
}
