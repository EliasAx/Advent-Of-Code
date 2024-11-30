def testPart1():
    inputFile = open("TestInputPart1.txt", "r")
    lines = inputFile.read().splitlines()

    answer = solution(lines)
    return answer, answer == 142

def solvePart1():
    inputFile = open("Input.txt", "r")
    lines = inputFile.read().splitlines()

    return solution(lines)

def solution(lines):
    sum = 0
    for line in lines:
        firstDigit = '-1'
        lastDigit = '-1'
        for char in line:
            if char.isdigit():
                if firstDigit == '-1':
                    firstDigit = char
                lastDigit = char
        numberChar = firstDigit+lastDigit
        sum += int(numberChar)

    return sum