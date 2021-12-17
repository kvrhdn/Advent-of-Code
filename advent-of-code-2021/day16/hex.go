package day16

import (
	"strconv"
	"strings"
)

func hexToBinary(hex string) string {
	var binary strings.Builder

	// lolsob
	for _, r := range hex {
		switch r {
		case '0':
			binary.WriteString("0000")
		case '1':
			binary.WriteString("0001")
		case '2':
			binary.WriteString("0010")
		case '3':
			binary.WriteString("0011")
		case '4':
			binary.WriteString("0100")
		case '5':
			binary.WriteString("0101")
		case '6':
			binary.WriteString("0110")
		case '7':
			binary.WriteString("0111")
		case '8':
			binary.WriteString("1000")
		case '9':
			binary.WriteString("1001")
		case 'A':
			binary.WriteString("1010")
		case 'B':
			binary.WriteString("1011")
		case 'C':
			binary.WriteString("1100")
		case 'D':
			binary.WriteString("1101")
		case 'E':
			binary.WriteString("1110")
		case 'F':
			binary.WriteString("1111")
		}
	}

	return binary.String()
}

func parseBinary(binary string) int {
	value, err := strconv.ParseInt(binary, 2, 32)
	if err != nil {
		panic("could not parse " + binary + " as a binary value: " + err.Error())
	}
	return int(value)
}

func parseBinaryInt64(binary string) int64 {
	value, err := strconv.ParseInt(binary, 2, 64)
	if err != nil {
		panic("could not parse " + binary + " as a binary value: " + err.Error())
	}
	return value
}
