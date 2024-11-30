from part1 import *
from part2 import *

def main():
    testAnswer, testPassed = testPart1()
    if testPassed:
        answer = solvePart1()
        print("Answer part1: ", answer)
    else:
        print("Part1 test failed with ", testAnswer)



    testAnswer, testPassed = testPart2()
    if testPassed:
        answer = solvePart2()
        print("Answer part2: ", answer)
    else:
        print("Part2 test failed with ", testAnswer)

if __name__ == "__main__":
    main()