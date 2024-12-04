def main():
    inputFile = open("Input.txt", "r")
    lines = inputFile.read().splitlines()

    part1(lines)
    part2(lines)


def part1(lines):
    sum = 0
    rowIndex = 0
    for line in lines:
        charIndex = 0
        for char in line:
            if char == "S":
                if charIndex >= 3:
                    if wordIsLeft(line, charIndex):
                        sum += 1
                if len(line) - charIndex > 3:
                    if wordIsRight(line, charIndex):
                        sum += 1
                if rowIndex >= 3:
                    if wordIsUp(lines, rowIndex, charIndex):
                        sum += 1
                if len(lines) - rowIndex > 3:
                    if wordIsDown(lines, rowIndex, charIndex):
                        sum += 1
                if rowIndex >= 3 and charIndex >= 3:
                    if wordIsDiagonalUpLeft(lines, rowIndex, charIndex):
                        sum += 1
                if rowIndex >= 3 and len(line) - charIndex > 3:
                    if wordIsDiagonalUpRight(lines, rowIndex, charIndex):
                        sum += 1
                if len(lines) - rowIndex > 3 and charIndex >= 3:
                    if wordIsDiagonalDownLeft(lines, rowIndex, charIndex):
                        sum += 1
                if len(lines) - rowIndex > 3 and len(line) - charIndex > 3:
                    if wordIsDiagonalDownRight(lines, rowIndex, charIndex):
                        sum += 1

            charIndex += 1

        rowIndex += 1

    print(sum)


def wordIsLeft(line, index):
    if line[index - 1] != "A":
        return False
    if line[index - 2] != "M":
        return False
    if line[index - 3] != "X":
        return False
    return True


def wordIsRight(line, index):
    if line[index + 1] != "A":
        return False
    if line[index + 2] != "M":
        return False
    if line[index + 3] != "X":
        return False
    return True


def wordIsUp(lines, rowIndex, charIndex):
    if lines[rowIndex - 1][charIndex] != "A":
        return False
    if lines[rowIndex - 2][charIndex] != "M":
        return False
    if lines[rowIndex - 3][charIndex] != "X":
        return False
    return True


def wordIsDown(lines, rowIndex, charIndex):
    if lines[rowIndex + 1][charIndex] != "A":
        return False
    if lines[rowIndex + 2][charIndex] != "M":
        return False
    if lines[rowIndex + 3][charIndex] != "X":
        return False
    return True


def wordIsDiagonalUpLeft(lines, rowIndex, charIndex):
    if lines[rowIndex - 1][charIndex - 1] != "A":
        return False
    if lines[rowIndex - 2][charIndex - 2] != "M":
        return False
    if lines[rowIndex - 3][charIndex - 3] != "X":
        return False
    return True


def wordIsDiagonalUpRight(lines, rowIndex, charIndex):
    if lines[rowIndex - 1][charIndex + 1] != "A":
        return False
    if lines[rowIndex - 2][charIndex + 2] != "M":
        return False
    if lines[rowIndex - 3][charIndex + 3] != "X":
        return False
    return True


def wordIsDiagonalDownLeft(lines, rowIndex, charIndex):
    if lines[rowIndex + 1][charIndex - 1] != "A":
        return False
    if lines[rowIndex + 2][charIndex - 2] != "M":
        return False
    if lines[rowIndex + 3][charIndex - 3] != "X":
        return False
    return True


def wordIsDiagonalDownRight(lines, rowIndex, charIndex):
    if lines[rowIndex + 1][charIndex + 1] != "A":
        return False
    if lines[rowIndex + 2][charIndex + 2] != "M":
        return False
    if lines[rowIndex + 3][charIndex + 3] != "X":
        return False
    return True


def part2(lines):
    sum = 0
    rowIndex = 0
    for line in lines:
        charIndex = 0
        for char in line:
            if char == "A":
                if (
                    charIndex >= 1
                    and len(line) - charIndex > 1
                    and rowIndex >= 1
                    and len(lines) - rowIndex > 1
                ):
                    if wordIsInX(lines, rowIndex, charIndex):
                        sum += 1

            charIndex += 1

        rowIndex += 1

    print(sum)


def wordIsInX(lines, rowIndex, charIndex):
    if (
        lines[rowIndex + 1][charIndex + 1] != "S"
        and lines[rowIndex + 1][charIndex + 1] != "M"
    ):
        return False
    if (
        lines[rowIndex + 1][charIndex - 1] != "S"
        and lines[rowIndex + 1][charIndex - 1] != "M"
    ):
        return False
    if (
        lines[rowIndex - 1][charIndex + 1] != "S"
        and lines[rowIndex - 1][charIndex + 1] != "M"
    ):
        return False
    if (
        lines[rowIndex - 1][charIndex - 1] != "S"
        and lines[rowIndex - 1][charIndex - 1] != "M"
    ):
        return False
    if lines[rowIndex + 1][charIndex + 1] == "S":
        if lines[rowIndex - 1][charIndex - 1] != "M":
            return False
    if lines[rowIndex + 1][charIndex + 1] == "M":
        if lines[rowIndex - 1][charIndex - 1] != "S":
            return False
    if lines[rowIndex + 1][charIndex - 1] == "M":
        if lines[rowIndex - 1][charIndex + 1] != "S":
            return False
    if lines[rowIndex + 1][charIndex - 1] == "S":
        if lines[rowIndex - 1][charIndex + 1] != "M":
            return False

    return True


if __name__ == "__main__":
    main()
