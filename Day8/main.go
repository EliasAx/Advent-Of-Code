package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var treeMatrix [][]int

func main() {

	readfile, err := os.Open("Input.txt")
	if err != nil {
		fmt.Println(err.Error())
	}
	fileScannerPart1 := bufio.NewScanner(readfile)
	fileScannerPart1.Split(bufio.ScanLines)

	buildTreeMatrix(fileScannerPart1)
	part1()
	part2()
}

func buildTreeMatrix(fileScanner *bufio.Scanner) {
	for fileScanner.Scan() {
		row := fileScanner.Text()
		rowTreeSizes := strings.Split(row, "")

		var rowTreeSizesInts []int
		for _, rowTreeSize := range rowTreeSizes {
			rowTreeSizeInt, err := strconv.Atoi(rowTreeSize)
			if err != nil {
				fmt.Println("Tree size is not a number: ", err.Error())
				return
			}
			rowTreeSizesInts = append(rowTreeSizesInts, rowTreeSizeInt)
		}

		treeMatrix = append(treeMatrix, rowTreeSizesInts)
	}
}

func part1() {

	treesVisible := 0
	treesVisible = len(treeMatrix) * 2
	treesVisible += (len(treeMatrix[0]) - 2) * 2

	for rowIndex := 1; rowIndex < len(treeMatrix)-1; rowIndex++ {
		for colIndex := 1; colIndex < len(treeMatrix[rowIndex])-1; colIndex++ {

			if treeIsVisible(rowIndex, colIndex) {
				//fmt.Println(fmt.Sprintf("[%d:%d] = %d", rowIndex, colIndex, treeMatrix[rowIndex][colIndex]))
				treesVisible++
			}
		}
	}

	fmt.Println(treesVisible)
}

func treeIsVisible(rowIndex, colIndex int) bool {
	treeSize := treeMatrix[rowIndex][colIndex]
	treeVisibleLeft := true
	for colIndex2 := 0; colIndex2 < colIndex; colIndex2++ {
		if treeSize <= treeMatrix[rowIndex][colIndex2] {
			treeVisibleLeft = false
		}
	}
	treeVisibleRight := true
	for colIndex2 := colIndex + 1; colIndex2 < len(treeMatrix[rowIndex]); colIndex2++ {
		if treeSize <= treeMatrix[rowIndex][colIndex2] {
			treeVisibleRight = false
		}
	}
	treeVisibleUp := true
	for rowIndex2 := 0; rowIndex2 < rowIndex; rowIndex2++ {
		if treeSize <= treeMatrix[rowIndex2][colIndex] {
			treeVisibleUp = false
		}
	}
	treeVisibleDown := true
	for rowIndex2 := rowIndex + 1; rowIndex2 < len(treeMatrix); rowIndex2++ {
		if treeSize <= treeMatrix[rowIndex2][colIndex] {
			treeVisibleDown = false
		}
	}
	return treeVisibleLeft || treeVisibleRight || treeVisibleUp || treeVisibleDown
}

func part2() {

	bestScenicScore := 0

	for rowIndex := 1; rowIndex < len(treeMatrix)-1; rowIndex++ {
		for colIndex := 1; colIndex < len(treeMatrix[rowIndex])-1; colIndex++ {
			scenicScore := scenicScore(rowIndex, colIndex)
			if scenicScore > bestScenicScore {
				bestScenicScore = scenicScore
			}
		}
	}

	fmt.Println(bestScenicScore)
}

func scenicScore(rowIndex, colIndex int) int {
	treeSize := treeMatrix[rowIndex][colIndex]
	numberOfTreesVisisbleLeft := 0
	for colIndex2 := colIndex - 1; colIndex2 >= 0; colIndex2-- {
		numberOfTreesVisisbleLeft++
		if treeSize <= treeMatrix[rowIndex][colIndex2] {
			break
		}
	}
	numberOfTreesVisisbleRight := 0
	for colIndex2 := colIndex + 1; colIndex2 < len(treeMatrix[rowIndex]); colIndex2++ {
		numberOfTreesVisisbleRight++
		if treeSize <= treeMatrix[rowIndex][colIndex2] {
			break
		}
	}
	numberOfTreesVisisbleUp := 0
	for rowIndex2 := rowIndex - 1; rowIndex2 >= 0; rowIndex2-- {
		numberOfTreesVisisbleUp++
		if treeSize <= treeMatrix[rowIndex2][colIndex] {
			break
		}
	}
	numberOfTreesVisisbleDown := 0
	for rowIndex2 := rowIndex + 1; rowIndex2 < len(treeMatrix); rowIndex2++ {
		numberOfTreesVisisbleDown++
		if treeSize <= treeMatrix[rowIndex2][colIndex] {
			break
		}
	}
	return numberOfTreesVisisbleLeft * numberOfTreesVisisbleRight * numberOfTreesVisisbleUp * numberOfTreesVisisbleDown
}
