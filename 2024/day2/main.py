def main():
    inputFile = open("Input.txt", "r")
    lines = inputFile.read().splitlines()

    part1(lines)
    part2(lines)

def part1(lines):
    safeReports = 0
    for report in lines:
        levelsStr = report.split(" ")
        levels = convertToInt(levelsStr)

        if isReportSafe(levels):
           safeReports += 1
        
    print(safeReports)


def part2(lines):
    safeReports = 0
    for report in lines:
        levelsStr = report.split(" ")
        levels = convertToInt(levelsStr)
        if not isReportSafe(levels):
            for index in range(0, len(levels)):
                testLevels = levels.copy()
                del testLevels[index]
                if isReportSafe(testLevels):
                    safeReports += 1
                    break
        else:
            safeReports += 1
                
    print(safeReports)

def isReportSafe(levels):
    increasing = (levels[0] < levels[1])
    for index in range(1, len(levels)):
        isIncreasing = (levels[index-1] < levels[index])
        if levels[index-1] == levels[index]:
            return False
        if increasing != isIncreasing:
            return False
        if abs(levels[index-1] - levels[index]) > 3:
            return False
        if abs(levels[index-1] - levels[index]) < 1:
            return False
    return True

def convertToInt(list):
    intList = []
    for elem in list:
        intList.append(int(elem))
    return intList

if __name__ == "__main__":
    main()
