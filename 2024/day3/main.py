import re


def main():
    inputFile = open("Input.txt", "r")
    input = inputFile.read()

    part1(input)
    part2(input)


def part1(input):
    occurrences = re.findall("(mul\\()(\\d+,\\d+)\\)", input)
    sum = 0
    for occurence in occurrences:
        numbers = occurence[1].split(",")
        score = int(numbers[0]) * int(numbers[1])
        sum += score

    print(sum)


def part2(input):
    occurrences = re.finditer("(mul\\()(\\d+,\\d+)\\)", input)
    doOccurrences = re.finditer("do\\(\\)", input)
    dontOccurrences = re.finditer("don't\\(\\)", input)

    doIndexes = [0]
    dontIndexes = []
    for o in doOccurrences:
        doIndexes.append(o.span()[1])
    for o in dontOccurrences:
        dontIndexes.append(o.span()[1])

    sum = 0
    for o in occurrences:
        index = o.span()[1]
        match = o.group()
        match = match.split("(")[1].removesuffix(")")
        if indexIsInDo(index, doIndexes, dontIndexes):
            numbers = match.split(",")
            score = int(numbers[0]) * int(numbers[1])
            sum += score

    print(sum)


def indexIsInDo(index, doIndexes, dontIndexes):
    closestDo = 1000000
    closestDont = 10000000
    for doIndex in doIndexes:
        if doIndex < index:
            closestDo = index - doIndex
    for dontIndex in dontIndexes:
        if dontIndex < index:
            closestDont = index - dontIndex

    return closestDo < closestDont


if __name__ == "__main__":
    main()
