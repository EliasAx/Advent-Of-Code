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
    sum = 0
    direction = (-1, 0)
    obstaclesHit = dict()
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
            obstaclesHit[(nextRow, nextCol)] = direction
            direction = changeDirection(direction)
            if len(obstaclesHit) >= 3:
                placements = placeForObstacle(
                    map, currentPosition, direction, obstaclesHit
                )
                if len(placements) > 0:
                    obstaclePositions.extend(placements)
                sum += len(placements)

            continue

        currentPosition = (nextRow, nextCol)

    print(obstaclePositions)
    print(sum)


def placeForObstacle(map, currentPosition, direction, obstaclesHit):
    placements = []
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
            break
        currentPosition = (nextRow, nextCol)
        if canHitObstacleSameDirection(
            map, currentPosition, changeDirection(direction), obstaclesHit
        ):
            placements.append(
                (currentPosition[0] + direction[0], currentPosition[1] + direction[1])
            )

    return placements


def canHitObstacleSameDirection(map, currentPosition, direction, obstaclesHit):
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
            key = (nextRow, nextCol)
            if key in obstaclesHit:
                if obstaclesHit[key] == direction:
                    return True
                else:
                    return False
            else:
                return False
        currentPosition = (nextRow, nextCol)

    return False


if __name__ == "__main__":
    main()
