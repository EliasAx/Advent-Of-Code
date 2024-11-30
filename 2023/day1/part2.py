import re
from itertools import groupby

def testPart2():
    inputFile = open("TestInputPart2.txt", "r")
    lines = inputFile.read().splitlines()

    answer = solution(lines)
    return answer, answer == 281

def solvePart2():
    inputFile = open("Input.txt", "r")
    lines = inputFile.read().splitlines()

    return solution(lines)

def solution(lines):

    for k, j in groupby("two1ninethree4fivesix66", str.isdigit):
        print(k)
        print(''.join(j).strip())
    print("----------------------------------------------------")

    numbersInText = ["one","two","three","four","five","six","seven","eight","nine"]
    sum = 0
    for line in lines:
        res = [''.join(j).strip() for k, j in groupby(line, str.isdigit)]
        print("ITEMS: ", res)
        res2 = re.split('one|two|three|four|five|six|seven|eight|nine', res[0])
        print("ITEMS2: ", res2)
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

def something(self):
    print(self)
    numbersInText = ["one","two","three","four","five","six","seven","eight","nine"]
    if any(numText in self for numText in numbersInText):
        #print("true")
        return True
    #print("false")
    return False