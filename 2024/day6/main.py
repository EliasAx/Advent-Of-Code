from copy import copy, deepcopy


def main():
    inputFile = open("Input.txt", "r")
    lines = inputFile.read().splitlines()

    map = []
    currentPosition = (-10, -10)
    for rowIndex in range(len(lines)):
        map.append(list(lines[rowIndex]))
        for colIndex in range(len(lines[rowIndex])):
            if lines[rowIndex][colIndex] == "^":
                currentPosition = (rowIndex, colIndex)
                break

    part1(deepcopy(map), currentPosition)
    part2(map, currentPosition)


def part1(map, currentPosition):
    visited = 0
    direction = (-1, 0)
    while True:
        currentRow = currentPosition[0]
        currentCol = currentPosition[1]
        nextRow = currentRow + direction[0]
        nextCol = currentCol + direction[1]
        if map[currentRow][currentCol] != "X":
            map[currentRow][currentCol] = "X"
            visited += 1
        if nextRow < 0 or nextRow >= len(map):
            break
        elif nextCol < 0 or nextCol >= len(map[nextRow]):
            break
        if map[nextRow][nextCol] == "#":
            direction = changeDirection(direction)
            continue
        currentPosition = (nextRow, nextCol)

    print(visited)


def changeDirection(direction):
    if direction[0] == 1:
        direction = (0, -1)
    elif direction[0] == -1:
        direction = (0, 1)
    elif direction[1] == 1:
        direction = (1, 0)
    elif direction[1] == -1:
        direction = (-1, 0)

    return direction


def part2(map, currentPosition):
    startPos = currentPosition
    direction = (-1, 0)
    startDir = (-1, 0)
    obstacleLoopPositions = []
    while True:
        currentRow = currentPosition[0]
        currentCol = currentPosition[1]
        nextRow = currentRow + direction[0]
        nextCol = currentCol + direction[1]
        if nextRow < 0 or nextRow >= len(map):
            break
        elif nextCol < 0 or nextCol >= len(map[nextRow]):
            break
        if map[nextRow][nextCol] == "#":
            direction = changeDirection(direction)
            continue
        else:
            if obstacleInDirection(map, currentPosition, changeDirection(direction)):
                if (
                    currentPosition[0] + direction[0],
                    currentPosition[1] + direction[1],
                ) not in obstacleLoopPositions:
                    newMap = deepcopy(map)
                    newMap[currentPosition[0] + direction[0]][
                        currentPosition[1] + direction[1]
                    ] = "#"
                    if isInLoop(newMap, startPos, startDir):
                        obstacleLoopPositions.append(
                            (
                                currentPosition[0] + direction[0],
                                currentPosition[1] + direction[1],
                            )
                        )

        currentPosition = (nextRow, nextCol)

    print(len(obstacleLoopPositions))


def isInLoop(map, currentPosition, direction):
    obstaclesHit = dict()
    while True:
        currentRow = currentPosition[0]
        currentCol = currentPosition[1]
        nextRow = currentRow + direction[0]
        nextCol = currentCol + direction[1]
        if nextRow < 0 or nextRow >= len(map):
            return False
        elif nextCol < 0 or nextCol >= len(map[nextRow]):
            return False
        if map[nextRow][nextCol] == "#":
            key = (nextRow, nextCol)
            if key in obstaclesHit:
                if obstaclesHit[key] == direction:
                    return True
            obstaclesHit[key] = direction
            direction = changeDirection(direction)
            continue
        currentPosition = (nextRow, nextCol)


def obstacleInDirection(map, currentPosition, direction):
    while True:
        currentRow = currentPosition[0]
        currentCol = currentPosition[1]
        nextRow = currentRow + direction[0]
        nextCol = currentCol + direction[1]
        if nextRow < 0 or nextRow >= len(map):
            break
        elif nextCol < 0 or nextCol >= len(map[nextRow]):
            break
        if map[nextRow][nextCol] == "#":
            return True
        currentPosition = (nextRow, nextCol)
    return False


if __name__ == "__main__":
    main()
