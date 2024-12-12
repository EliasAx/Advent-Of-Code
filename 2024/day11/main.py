from copy import deepcopy
import time


def main():
    inputFile = open("Input.txt", "r")
    lines = inputFile.read().splitlines()
    stones = []
    for stone in lines[0].split(" "):
        stones.append(int(stone))

    part1(deepcopy(stones))
    start = time.time()
    part2(stones)
    end = time.time()
    print(end - start)


def part1(stones):
    for i in range(25):
        blink(stones)
    print(len(stones))


def blink(stones):
    stoneIndex = 0
    while stoneIndex < len(stones):
        stone = stones[stoneIndex]
        if stone == 0:
            stones[stoneIndex] = 1
        elif len(str(stone)) % 2 == 0:
            stone1 = str(stone)[: int(len(str(stone)) / 2)]
            stone2 = str(stone)[int(len(str(stone)) / 2) :]
            stones[stoneIndex] = int(stone1)
            stones.insert(stoneIndex + 1, int(stone2))
            stoneIndex += 1
        else:
            stones[stoneIndex] = stone * 2024
        stoneIndex += 1


def part2(stones):
    smartStones = dict()
    for stone in stones:
        if stone in smartStones:
            smartStones[stone] += 1
        else:
            smartStones[stone] = 1
    for i in range(75):
        smartStones = betterBlink(smartStones)
    sum = 0
    for stone in smartStones:
        sum += smartStones[stone]
    print(sum)


def betterBlink(oldStones):
    stones = deepcopy(oldStones)
    for stone in oldStones:
        if stone == 0:
            if 1 not in stones:
                stones[1] = oldStones[stone]
            else:
                stones[1] += oldStones[stone]
        elif len(str(stone)) % 2 == 0:
            stone1 = int(str(stone)[: int(len(str(stone)) / 2)])
            stone2 = int(str(stone)[int(len(str(stone)) / 2) :])
            if stone1 not in stones:
                stones[stone1] = oldStones[stone]
            else:
                stones[stone1] += oldStones[stone]
            if stone2 not in stones:
                stones[stone2] = oldStones[stone]
            else:
                stones[stone2] += oldStones[stone]
        else:
            if stone * 2024 not in stones:
                stones[stone * 2024] = oldStones[stone]
            else:
                stones[stone * 2024] += oldStones[stone]
        stones[stone] -= oldStones[stone]
        if stones[stone] == 0:
            del stones[stone]

    return stones


if __name__ == "__main__":
    main()
