import json
import copy

def main():
    inputFile = open("Input.txt", "r")
    lines = inputFile.read().splitlines()

    pairs = []

    currentPair = Pair([], [])
    index = 0
    for line in lines:
        if line == "":
            pairs.append(currentPair)
            currentPair = Pair([], [])
            continue

        test = json.loads(line)
        if index % 2 == 0:
            currentPair.packet1 = test
        else:
            currentPair.packet2 = test
        index += 1

    #Add the last pair
    pairs.append(currentPair)

    index = 1
    sumindicies = 0
    for pair in pairs:
        # print(pair.packet1)
        # print(pair.packet2)
        # print()
        if comparePackets(pair.packet1, pair.packet2):
            sumindicies += index
        index+=1

    print("Part1: ", sumindicies)

    #---Part2----
    allPackets = []
    for line in lines:
        if line == "":
            continue

        test = json.loads(line)
        allPackets.append(test)

    #Add divider packets
    allPackets.append([[2]])
    allPackets.append([[6]])

    for i in range(len(allPackets)):
        for j in range(0, len(allPackets)-i-1):
            packet1 = copy.deepcopy(allPackets[j])
            packet2 = copy.deepcopy(allPackets[j+1])
            if not comparePackets(packet1, packet2):
                allPackets[j], allPackets[j + 1] = allPackets[j + 1], allPackets[j]

    indexDivider1 = 0
    indexDivider2 = 0
    index = 1
    for p in allPackets:
        if p == [[2]]:
            indexDivider1 = index
        if p == [[6]]:
            indexDivider2 = index
            break
        index += 1

    print("Part 2: ", indexDivider1*indexDivider2)




def comparePackets(packet1, packet2):
    # print("Currently comparing")
    # print(packet1)
    # print(packet2)
    largest = len(packet1)
    if len(packet2) > largest:
        largest = len(packet2)
    for index in range(largest):
        if len(packet2)-1 < index:
            return False
        if len(packet1)-1 < index:
            return True
        if isinstance(packet1[index], type(packet2[index])):
            if isinstance(packet1[index], list):
                compared = comparePackets(packet1[index], packet2[index])
                if compared is None:
                    continue
                else:
                    return compared
            else:
                #compare numbers
                if packet1[index] < packet2[index]:
                    return True
                elif packet1[index] > packet2[index]:
                    return False
        else:
            if isinstance(packet1[index], int):
                packet1[index] = [packet1[index]]
            elif isinstance(packet2[index], int):
                packet2[index] = [packet2[index]]

            compared = comparePackets(packet1[index], packet2[index])
            if compared is None:
                continue
            else:
                return compared


class Pair():
    def __init__(self, packet1, packet2):
        super(Pair, self).__init__()
        self.packet1 = packet1
        self.packet2 = packet2


if __name__ == "__main__":
    main()