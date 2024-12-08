from copy import deepcopy


def main():
    inputFile = open("Input.txt", "r")
    lines = inputFile.read().splitlines()

    antennas = dict()
    for rowIndex in range(len(lines)):
        for colIndex in range(len(lines[rowIndex])):
            if lines[rowIndex][colIndex] != ".":
                key = lines[rowIndex][colIndex]
                antennas.setdefault(key, []).append((rowIndex, colIndex))

    part1(deepcopy(antennas), len(lines), len(lines[0]))
    part2(antennas, len(lines), len(lines[0]))


def part1(antennas, sizeRows, sizeCols):
    usedPos = dict()
    antinodes = []
    for key in antennas:
        for currentPos in antennas[key]:
            for pos in antennas[key]:
                if currentPos == pos:
                    continue
                diff = posDiff(currentPos, pos)
                antinodePos = posAdd(currentPos, diff)
                if (
                    inBounds(antinodePos, sizeRows, sizeCols)
                    and antinodePos not in usedPos
                ):
                    antinodes.append(antinodePos)
                    usedPos[antinodePos] = True

    print(len(antinodes))


def posDiff(pos1, pos2):
    return (pos1[0] - pos2[0], pos1[1] - pos2[1])


def posAdd(pos1, pos2):
    return (pos1[0] + pos2[0], pos1[1] + pos2[1])


def inBounds(pos, sizeRows, sizeCols):
    if pos[0] < 0 or pos[0] >= sizeRows:
        return False
    if pos[1] < 0 or pos[1] >= sizeCols:
        return False
    return True


def part2(antennas, sizeRows, sizeCols):
    usedPos = dict()
    antinodes = []
    for key in antennas:
        for currentPos in antennas[key]:
            if currentPos not in usedPos:
                antinodes.append(currentPos)
                usedPos[currentPos] = True

            for pos in antennas[key]:
                if currentPos == pos:
                    continue
                diff = posDiff(currentPos, pos)
                antinodePos = posAdd(currentPos, diff)
                while inBounds(antinodePos, sizeRows, sizeCols):
                    if antinodePos not in usedPos:
                        antinodes.append(antinodePos)
                        usedPos[antinodePos] = True
                    antinodePos = posAdd(antinodePos, diff)

    print(len(antinodes))


if __name__ == "__main__":
    main()
