DEBUG = False

def main():
    inputFile = open("Input.txt", "r")
    lines = inputFile.read().splitlines()

    rope = [{"x":int(10), "y":int(10)} for i in range(10)]

    part1(lines, rope)


def part1(lines, rope):
    tailIndex = len(rope)-1

    positions = set()
    positions.add(tuple(rope[tailIndex].values()))

    i = 0
    for line in lines:
        movement = line.split(" ")
        if movement[0] == "R":
            for _ in range(0, int(movement[1])):
                debugPrint(rope, "MOVING RIGHT")
                rope[0]["x"] += 1 # Move head first
                for knotIndex in range(1, len(rope)):
                    moveTail(rope[knotIndex-1], rope[knotIndex], i)
                positions.add(tuple(rope[tailIndex].values()))
        if movement[0] == "L":
            for _ in range(0, int(movement[1])):
                debugPrint(rope, "MOVING LEFT")
                rope[0]["x"] -= 1 # Move head first
                for knotIndex in range(1, len(rope)):
                    moveTail(rope[knotIndex-1], rope[knotIndex], i)
                positions.add(tuple(rope[tailIndex].values()))
        if movement[0] == "U":
            for _ in range(0, int(movement[1])):
                debugPrint(rope, "MOVING UP")
                rope[0]["y"] += 1 # Move head first
                for knotIndex in range(1, len(rope)):
                    moveTail(rope[knotIndex-1], rope[knotIndex], i)
                positions.add(tuple(rope[tailIndex].values()))
        if movement[0] == "D":
            for _ in range(0, int(movement[1])):
                debugPrint(rope, "MOVING DOWN")
                rope[0]["y"] -= 1 # Move head first
                for knotIndex in range(1, len(rope)):
                    moveTail(rope[knotIndex-1], rope[knotIndex], i)
                positions.add(tuple(rope[tailIndex].values()))
        i += 1
        if DEBUG:
            print("Current: ", i)
        # debugPrint(rope)

    print(len(positions))

def debugPrint(rope, movement):
    if DEBUG:
        print("\n")
        printPosition(rope, movement)

def printPosition(rope, movement):
    print(rope)
    print(movement)
    matrix = [["."for i in range(60)]for i in range(60)]
    matrix[10][10] = "s"
    for index in reversed(range(len(rope))):
        if index == 0:
            matrix[rope[index]["y"]][rope[index]["x"]] = "H"
        elif index == len(rope)-1:
            matrix[rope[index]["y"]][rope[index]["x"]] = "T"
        else:
            matrix[rope[index]["y"]][rope[index]["x"]] = str(index)


    for rowIndex in reversed(range(len(matrix))):
        for colIndex in range(len(matrix[rowIndex])):
            print(matrix[rowIndex][colIndex], end = '')
        print("")



def moveTail(headPosition, tailPosition, currentIndex):
    xDiff = headPosition["x"] - tailPosition["x"]
    yDiff = headPosition["y"] - tailPosition["y"]

    tailMovement = {"x": 0, "y": 0}

    if xDiff > 1 or xDiff < -1:
        if yDiff == 0:
            tailMovement = {"x": int(xDiff/2), "y": 0}
        elif yDiff > 1 or yDiff < -1:
            tailMovement = {"x": int(xDiff/2), "y": int(yDiff/2)}
        else:
            tailMovement = {"x": int(xDiff/2), "y": yDiff}
    elif yDiff > 1 or yDiff < -1:
        if xDiff == 0:
            tailMovement = {"x": 0, "y": int(yDiff/2)}
        elif xDiff > 1 or xDiff < -1:
            tailMovement = {"x": int(xDiff/2), "y": int(yDiff/2)}
        else:
            tailMovement = {"x": xDiff, "y": int(yDiff/2)}

    tailPosition["x"] += tailMovement["x"]
    tailPosition["y"] += tailMovement["y"]




if __name__ == "__main__":
    main()