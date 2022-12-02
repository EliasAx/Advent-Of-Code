package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

var pointOutcomes map[string]map[string]int

func main() {
	setup()
	readfile, err := os.Open("Input.txt")
	if err != nil {
		fmt.Println(err.Error())
	}
	fileScannerPart1 := bufio.NewScanner(readfile)
	fileScannerPart1.Split(bufio.ScanLines)

	readfile, err = os.Open("Input.txt")
	if err != nil {
		fmt.Println(err.Error())
	}
	fileScannerPart2 := bufio.NewScanner(readfile)
	fileScannerPart2.Split(bufio.ScanLines)

	part1(fileScannerPart1)
	part2(fileScannerPart2)
}

func setup() {
	pointOutcomes = make(map[string]map[string]int)
	pointOutcomes["A"] = make(map[string]int)
	pointOutcomes["B"] = make(map[string]int)
	pointOutcomes["C"] = make(map[string]int)

	pointOutcomes["A"]["X"] = 4
	pointOutcomes["A"]["Y"] = 8
	pointOutcomes["A"]["Z"] = 3
	pointOutcomes["B"]["X"] = 1
	pointOutcomes["B"]["Y"] = 5
	pointOutcomes["B"]["Z"] = 9
	pointOutcomes["C"]["X"] = 7
	pointOutcomes["C"]["Y"] = 2
	pointOutcomes["C"]["Z"] = 6
}

func part1(fileScanner *bufio.Scanner) {
	totalScore := 0

	for fileScanner.Scan() {
		row := fileScanner.Text()
		rockPaperScissors := strings.Split(row, " ")
		totalScore += pointOutcomes[rockPaperScissors[0]][rockPaperScissors[1]]
	}

	fmt.Println("Part1 Score: ", totalScore)
}

func part2(fileScanner *bufio.Scanner) {
	totalScore := 0

	for fileScanner.Scan() {
		row := fileScanner.Text()
		rockPaperScissors := strings.Split(row, " ")
		totalScore += pointOutcomes[rockPaperScissors[0]][chooseRockPaperScissor(rockPaperScissors[0], rockPaperScissors[1])]
	}

	fmt.Println("Part2 Score: ", totalScore)
}

func chooseRockPaperScissor(rockPaperScissor string, outcome string) (choice string) {
	switch outcome {
	case "X":
		for key, val := range pointOutcomes[rockPaperScissor] {
			if val <= 3 {
				return key
			}
		}
	case "Y":
		for key, val := range pointOutcomes[rockPaperScissor] {
			if val > 3 && val <= 6 {
				return key
			}
		}
	case "Z":
		for key, val := range pointOutcomes[rockPaperScissor] {
			if val > 6 {
				return key
			}
		}
	}
	fmt.Println("Something went wrong...")
	return ""
}
