def main():
    inputFile = open("Input.txt", "r")
    lines = inputFile.read().splitlines()

    part1(lines)
    part2(lines)

def part1(lines):
    leftList = []
    rightList = []
    for line in lines:
        separation = line.split("   ")
        leftList.append(int(separation[0]))
        rightList.append(int(separation[1]))
    leftList.sort()
    rightList.sort()

    sum = 0
    for index in range(0, len(leftList)):
       distanceBetween = abs(leftList[index]-rightList[index])
       sum += distanceBetween
    print(sum)



def part2(lines):
    leftList = []
    rightList = []
    for line in lines:
        separation = line.split("   ")
        leftList.append(int(separation[0]))
        rightList.append(int(separation[1]))

    similarityScore = 0
    for leftNumber in leftList:
        appearances = 0
        for rightNumber in rightList:
            if leftNumber == rightNumber:
                appearances += 1

        similarityScore += leftNumber * appearances

    print(similarityScore)

if __name__ == "__main__":
    main()
