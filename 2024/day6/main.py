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

    part1(map, currentPosition)
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
    sum = 0
    direction = (-1, 0)
    obstaclesHit = []
    obstaclePositions = []
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
            obstaclesHit.append((nextRow, nextCol))
            if len(obstaclesHit) >= 3:
                pos = getObstaclePositionForLoop(
                    currentPosition,
                    direction,
                    obstaclesHit[len(obstaclesHit) - 3],
                )
                if not thereIsObstacleBetweenPoints(
                    map, currentPosition, pos, direction
                ):
                    if not pos in obstaclePositions:
                        obstaclePositions.append(pos)
                        sum += 1
                if direction[1] == -1:
                    pos = (currentPosition[0], startPos[1] + direction[1])
                    if not thereIsObstacleBetweenPoints(
                        map, currentPosition, pos, direction
                    ):
                        if not pos in obstaclePositions:
                            obstaclePositions.append(pos)
                            sum += 1

            continue

        currentPosition = (nextRow, nextCol)

    print(obstaclePositions)
    print(sum)


def getObstaclePositionForLoop(currentPosition, direction, obstacleForLoopPos):
    if direction[0] != 0:
        return (obstacleForLoopPos[0] + direction[0], currentPosition[1])
    elif direction[1] != 0:
        return (currentPosition[0], obstacleForLoopPos[1] + direction[1])


def thereIsObstacleBetweenPoints(map, currentPosition, obstaclePosition, direction):
    while True:
        currentRow = currentPosition[0]
        currentCol = currentPosition[1]
        nextRow = currentRow + direction[0]
        nextCol = currentCol + direction[1]
        if (nextRow, nextCol) == obstaclePosition:
            break
        if map[nextRow][nextCol] == "#":
            return True
        currentPosition = (nextRow, nextCol)

    return False


if __name__ == "__main__":
    main()
