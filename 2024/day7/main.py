from copy import deepcopy


def main():
    inputFile = open("Input.txt", "r")
    lines = inputFile.read().splitlines()
    result = []
    numbers = []
    for line in lines:
        split = line.split(":")
        result.append(int(split[0]))
        numbers.append([int(x) for x in split[1].strip().split(" ")])

    part1(deepcopy(result), deepcopy(numbers))
    part2(result, numbers)


def part1(result, numbers):
    combinations = dict()
    for index in range(len(numbers)):
        operatorSpots = len(numbers[index]) - 1

        currentCombinations = []
        currentCombinations.append([numbers[index][0], "+", numbers[index][1]])
        currentCombinations.append([numbers[index][0], "*", numbers[index][1]])
        for opSpot in range(1, operatorSpots):
            forLooping = deepcopy(currentCombinations)
            for currentCombinationIndex in range(len(forLooping)):
                cu = deepcopy(forLooping[currentCombinationIndex])
                cu.extend(["*", numbers[index][opSpot + 1]])
                currentCombinations[currentCombinationIndex].extend(
                    ["+", numbers[index][opSpot + 1]]
                )
                currentCombinations.append(cu)

        combinations[result[index]] = currentCombinations

    matchesResult(combinations)


def matchesResult(combination):
    successFul = 0
    for result in combination:
        calculations = combination[result]
        for calc in calculations:
            numberOfOperations = int((len(calc) - 1) / 2)

            for opa in range(numberOfOperations):
                firstNumber = calc[0]
                secondNumber = calc[2]
                operator = calc[1]
                del calc[2]
                del calc[1]
                if operator == "+":
                    calc[0] = firstNumber + secondNumber
                elif operator == "*":
                    calc[0] = firstNumber * secondNumber
                else:
                    calc[0] = int(str(firstNumber) + str(secondNumber))
            if calc[0] == result:
                successFul += result
                break
    print(successFul)


def part2(result, numbers):
    combinations = dict()
    for index in range(len(numbers)):
        operatorSpots = len(numbers[index]) - 1

        currentCombinations = []
        currentCombinations.append([numbers[index][0], "+", numbers[index][1]])
        currentCombinations.append([numbers[index][0], "*", numbers[index][1]])
        currentCombinations.append([numbers[index][0], "||", numbers[index][1]])
        for opSpot in range(1, operatorSpots):
            forLooping = deepcopy(currentCombinations)
            for currentCombinationIndex in range(len(forLooping)):
                cu = deepcopy(forLooping[currentCombinationIndex])
                cu2 = deepcopy(forLooping[currentCombinationIndex])
                cu.extend(["*", numbers[index][opSpot + 1]])
                cu2.extend(["||", numbers[index][opSpot + 1]])
                currentCombinations[currentCombinationIndex].extend(
                    ["+", numbers[index][opSpot + 1]]
                )
                currentCombinations.append(cu)
                currentCombinations.append(cu2)

        combinations[result[index]] = currentCombinations

    matchesResult(combinations)


if __name__ == "__main__":
    main()
