from copy import deepcopy


def main():
    inputFile = open("Input.txt", "r")
    lines = inputFile.read().splitlines()
    map = []
    for line in lines:
        row = []
        for num in line:
            row.append(int(num))
        map.append(row)

    part1(deepcopy(map))
    part2(map)


def part1(lines):
    startPositions = []
    for rowIndex in range(len(lines)):
        for colIndex in range(len(lines[rowIndex])):
            if lines[rowIndex][colIndex] == 0:
                startPositions.append((rowIndex, colIndex))

    sum = 0
    for start in startPositions:
        num = recursion(lines, start, 0, 0, [], False)
        sum += num

    print(sum)


def recursion(map, pos, height, foundPaths, reached, allPossibilites):
    possiblePaths = []
    up = (pos[0] - 1, pos[1])
    down = (pos[0] + 1, pos[1])
    left = (pos[0], pos[1] - 1)
    right = (pos[0], pos[1] + 1)
    if isInBounds(map, up):
        if map[up[0]][up[1]] == height + 1:
            possiblePaths.append((up[0], up[1]))
    if isInBounds(map, down):
        if map[down[0]][down[1]] == height + 1:
            possiblePaths.append((down[0], down[1]))
    if isInBounds(map, left):
        if map[left[0]][left[1]] == height + 1:
            possiblePaths.append((left[0], left[1]))
    if isInBounds(map, right):
        if map[right[0]][right[1]] == height + 1:
            possiblePaths.append((right[0], right[1]))
    if len(possiblePaths) == 0:
        return foundPaths

    for path in possiblePaths:
        if height + 1 == 9:
            if not allPossibilites and path not in reached:
                # Part 1
                foundPaths += 1
                reached.append(path)
            elif allPossibilites:
                # Part 2
                foundPaths += 1
        else:
            foundPaths = recursion(
                map, path, height + 1, foundPaths, reached, allPossibilites
            )

    return foundPaths


def isInBounds(map, pos):
    if pos[0] < 0 or pos[0] >= len(map):
        return False
    if pos[1] < 0 or pos[1] >= len(map[0]):
        return False
    return True


def part2(lines):
    startPositions = []
    for rowIndex in range(len(lines)):
        for colIndex in range(len(lines[rowIndex])):
            if lines[rowIndex][colIndex] == 0:
                startPositions.append((rowIndex, colIndex))

    sum = 0
    for start in startPositions:
        num = recursion(lines, start, 0, 0, [], True)
        sum += num

    print(sum)


if __name__ == "__main__":
    main()
