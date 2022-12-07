from anytree import NodeMixin, RenderTree


def main():
    inputFile = open("Input.txt", "r")
    lines = inputFile.read().splitlines()

    root = Directory("/", 0)

    readIntoTree(lines, root)
    debugPrintTree(root)
    part1(root)
    part2(root)


def debugPrintTree(root):
    for pre, _, node in RenderTree(root):
        treestr = u"%s%s" % (pre, node.name)
        print(treestr.ljust(8), node.size)


def part1(root):
    answer = 0
    for pre, _, node in RenderTree(root):
        if len(node.children) != 0:
            if node.size < 100000:
                answer += node.size

    print("Answer part1: ", answer)


def part2(root):
    spaceNeeded = root.size + 30000000 - 70000000
    currentSmallest = 70000000
    for pre, _, node in RenderTree(root):
        if len(node.children) != 0:
            if node.size > spaceNeeded:
                if node.size < currentSmallest:
                    currentSmallest = node.size

    print("Answer part2: ", currentSmallest)


def readIntoTree(lines, root):
    current_directory = root

    for line in lines:
        if line[0] == '$':
            command = line[1:].strip()
            if "cd" in command:
                directory = command[2:].strip()
                if directory == "/":
                    current_directory = root
                elif directory == "..":
                    current_directory = current_directory.parent
                else:
                    for child in current_directory.children:
                        if child.name == directory:
                            current_directory = child
                            break
        else:
            if "dir" in line:
                dirName = line.split(" ")[1]
                Directory(dirName, 0, current_directory)
            else:
                splitLine = line.split(" ")
                fileSize = splitLine[0]
                fileName = splitLine[1]
                Directory(fileName, fileSize, current_directory)

                directory = current_directory
                while directory is not None:
                    directory.size += int(fileSize)
                    directory = directory.parent


class Directory(NodeMixin):
    def __init__(self, name, size, parent=None, children=None):
        super(Directory, self).__init__()
        self.name = name
        self.size = size
        self.parent = parent
        if children:  # set children only if given
            self.children = children


if __name__ == "__main__":
    main()
