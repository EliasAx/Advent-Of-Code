import string

def main():
    inputFile = open("Input.txt", "r")
    lines = inputFile.read().splitlines()

    part1(lines)
    part2(lines)


def part1(lines):
    priority = 0

    for line in lines:
        numberOfItems = len(line)
        compartment1 = line[0:int((numberOfItems/2))]
        compartment2 = line[int(numberOfItems/2):]
        for item in line:
            if item in compartment1 and item in compartment2:
                alphabetIndex = string.ascii_letters.index(item)
                priority += alphabetIndex+1
                break

    print(priority)

def part2(lines):
    priority = 0

    groups = [lines[x:x+3] for x in range(0, len(lines), 3)]

    for group in groups:
        elf1 = group[0]
        elf2 = group[1]
        elf3 = group[2]

        elf12Common = set(elf1).intersection(elf2)
        elf23Common = set(elf2).intersection(elf3)
        allCommon = set(elf12Common).intersection(elf23Common)
        alphabetIndex = string.ascii_letters.index(allCommon.pop())
        priority += alphabetIndex+1

    print(priority)

if __name__ == "__main__":
    main()