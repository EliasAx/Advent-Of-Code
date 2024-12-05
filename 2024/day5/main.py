def main():
    inputFile = open("Input.txt", "r")
    lines = inputFile.read().splitlines()
    index = 0
    rules = []
    updates = []
    for line in lines:
        if len(line) == 0:
            rules = lines[:index]
            updates = lines[index + 1 :]
            break
        index += 1
    part1(updates, rules)
    part2(updates, rules)


def part1(updates, rules):
    sum = 0
    for update in updates:
        pages = update.split(",")
        for index in range(len(pages)):
            page = pages[index]
            if not isPageCorrectlyOrdered(rules, pages, page, index):
                break

            if index == len(pages) - 1:
                # add middle page
                middleIndex = int((len(pages) - 1) / 2)
                sum += int(pages[middleIndex])

    print(sum)


def part2(updates, rules):
    sum = 0
    incorrectlyOrderedUpdates = []
    for update in updates:
        pages = update.split(",")
        for index in range(len(pages)):
            page = pages[index]
            if not isPageCorrectlyOrdered(rules, pages, page, index):
                incorrectlyOrderedUpdates.append(update)
                break

    for update in incorrectlyOrderedUpdates:
        ordered = orderUpdate(update, rules)
        middleIndex = int((len(ordered) - 1) / 2)
        sum += int(ordered[middleIndex])

    print(sum)


def findIndex(list, elem):
    for index in range(len(list)):
        if list[index] == elem:
            return index
    return -1


def isPageCorrectlyOrdered(rules, updatePages, page, currentPageIndex):
    for rule in rules:
        if page in rule:
            ruleParts = rule.split("|")
            indexOfRulePage = 0
            if page == ruleParts[0]:
                indexOfRulePage = findIndex(updatePages, ruleParts[1])
                if indexOfRulePage == -1:
                    continue
                if indexOfRulePage > currentPageIndex:
                    continue
                else:
                    return False
            elif page == ruleParts[1]:
                indexOfRulePage = findIndex(updatePages, ruleParts[0])
                if indexOfRulePage == -1:
                    continue
                if indexOfRulePage < currentPageIndex:
                    continue
                else:
                    return False
            else:
                return False
    return True


def orderUpdate(update, rules):
    pages = update.split(",")
    switches = -1
    while switches != 0:
        switches = 0
        for pageIndex in range(1, len(pages)):
            for rule in rules:
                leftRule = rule.split("|")[0]
                rightRule = rule.split("|")[1]
                if pages[pageIndex - 1] == rightRule and pages[pageIndex] == leftRule:
                    # Switch
                    tmp = pages[pageIndex - 1]
                    pages[pageIndex - 1] = pages[pageIndex]
                    pages[pageIndex] = tmp
                    switches += 1
    return pages


if __name__ == "__main__":
    main()
