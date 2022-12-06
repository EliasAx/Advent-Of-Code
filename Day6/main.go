package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	readfile, err := os.Open("Input.txt")
	if err != nil {
		fmt.Println(err.Error())
	}
	fileScannerPart1 := bufio.NewScanner(readfile)
	fileScannerPart1.Scan()
	input := fileScannerPart1.Text()

	part1And2(input, 4)
	part1And2(input, 14)
}

func part1And2(input string, numberOfUniqueChars int) {
	inputArray := strings.Split(input, "")
	for index, _ := range inputArray {
		if index+numberOfUniqueChars < len(input) {
			fourChars := inputArray[index : index+numberOfUniqueChars]
			if !hasDuplicates(fourChars) {
				fmt.Println(index + numberOfUniqueChars)
				break
			}
		}
	}
}

func contains(s []string, str string) bool {
	for _, v := range s {
		if v == str {
			return true
		}
	}

	return false
}

func hasDuplicates(s []string) bool {
	for index, char := range s {
		withoutChar := make([]string, len(s))
		copy(withoutChar, s)
		withoutChar = append(withoutChar[:index], withoutChar[index+1:]...)
		if contains(withoutChar, char) {
			return true
		}
	}
	return false
}
