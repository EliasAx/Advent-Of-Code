from copy import deepcopy


def main():
    inputFile = open("Input.txt", "r")
    lines = inputFile.read().splitlines()

    numbers = []
    numbersPart2 = []
    for index in range(len(lines[0])):
        numbers.append(int(lines[0][index]))
        numbersPart2.append((int(lines[0][index]), int(index / 2)))

    part1(deepcopy(numbers))
    part2(numbersPart2)


def part1(numbers):
    ordered = []
    index = 0
    while index < len(numbers):
        if index >= len(numbers) - 1:
            addIndexForNumbers(ordered, numbers[index], len(numbers) - 1)
            break
        currentNumber = numbers[index]
        if index % 2 == 1:
            lastNumber = numbers[len(numbers) - 1]
            numbers[len(numbers) - 1] = lastNumber - currentNumber
            if currentNumber <= lastNumber:
                addIndexForNumbers(ordered, currentNumber, len(numbers) - 1)
            else:
                addIndexForNumbers(ordered, lastNumber, len(numbers) - 1)
                del numbers[len(numbers) - 2]
                del numbers[len(numbers) - 1]
                numbers[index] = currentNumber - lastNumber
                continue
            if numbers[len(numbers) - 1] == 0:
                del numbers[len(numbers) - 2]
                del numbers[len(numbers) - 1]
        else:
            addIndexForNumbers(ordered, currentNumber, index)
        index += 1

    checksum = 0
    for index in range(len(ordered)):
        checksum += index * ordered[index]

    print(checksum)


def addIndexForNumbers(list, number, index):
    for whatev in range(number):
        list.append(int(index / 2))


def part2(numbers):

    index = len(numbers) - 1
    while index >= 0:
        currentDataNumber = numbers[index]
        emptySpaceIndex = 1
        emptySpaceNumber = numbers[emptySpaceIndex]
        while currentDataNumber[0] > emptySpaceNumber[0]:
            emptySpaceIndex += 2
            if emptySpaceIndex >= len(numbers):
                break
            emptySpaceNumber = numbers[emptySpaceIndex]

        if emptySpaceIndex < index:
            diff = emptySpaceNumber[0] - currentDataNumber[0]
            numbers[index] = (0, 0)
            numbers.insert(index + 1, (currentDataNumber[0], 0))
            numbers.insert(index + 2, (0, 0))
            numbers[emptySpaceIndex] = (0, 0)
            numbers.insert(emptySpaceIndex + 1, currentDataNumber)
            numbers.insert(emptySpaceIndex + 2, (diff, 0))

        else:
            index -= 2

    ordered = []
    for index in range(len(numbers)):
        if index % 2 == 0 or index == 0:
            addIndexForNumbers(ordered, numbers[index][0], numbers[index][1] * 2)
        else:
            addIndexForNumbers(ordered, numbers[index][0], 0)

    checksum = 0
    for index in range(len(ordered)):
        checksum += index * ordered[index]

    print(checksum)


if __name__ == "__main__":
    main()
