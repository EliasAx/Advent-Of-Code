def main():
    inputFile = open("Input.txt", "r")
    lines = inputFile.read().splitlines()

    get_overlap(lines)

def get_overlap(lines):
    numberOfPairsPart1 = 0
    numberOfPairsPart2 = 0
    for line in lines:
        ranges = line.split(",")

        tmp = ranges[0].split("-")
        range1 = range(int(tmp[0]), int(tmp[1])+1)
        tmp = ranges[1].split("-")
        range2 = range(int(tmp[0]), int(tmp[1])+1)

        #Part 1
        if range_subset(range1, range2) or range_subset(range2, range1):
            numberOfPairsPart1 += 1
        #Part 2
        if range_overlap(range1, range2) or range_overlap(range2, range1):
            numberOfPairsPart2 += 1

    print("Part 1: ", numberOfPairsPart1)
    print("Part 2: ", numberOfPairsPart2)


def range_subset(range1, range2):
    return range1.start in range2 and range1[-1] in range2

def range_overlap(range1, range2):
    return range1.start in range2 or range1[-1] in range2

if __name__ == "__main__":
    main()