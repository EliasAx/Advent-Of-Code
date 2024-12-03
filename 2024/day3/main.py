import re


def main():
    inputFile = open("TestInput2.txt", "r")
    input = inputFile.read()

    part1(input)
    part2(input)


def part1(input):
    occurrences = re.findall("(mul\\()(\\d+,\\d+)\\)", input)
    sum = 0
    for occurence in occurrences:
        numbers = occurence[1].split(",")
        print(numbers)
        score = int(numbers[0]) * int(numbers[1])
        sum += score

    print(sum)


def part2(input):
    occurrences = re.findall("(mul\\()(\\d+,\\d+)\\)", input)
    doOccurrences = re.search("do\\(\\)", input)
    dontOccurrences = re.findall("don't\\(\\).*(mul\\()(\\d+,\\d+)\\)", input)
    print(occurrences)
    print(doOccurrences)
    print(dontOccurrences)
    sum = 0
    # for occurence in occurrences:
    #     numbers = occurence[1].split(",")
    #     print(numbers)
    #     score = int(numbers[0]) * int(numbers[1])
    #     sum += score

    print(sum)
    return


if __name__ == "__main__":
    main()
