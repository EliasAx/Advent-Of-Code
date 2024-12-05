def main():
    inputFile = open("TestInput.txt", "r")
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

    for update in incorrectlyOrderedUpdates:
        orderUpdate(update, rules)

    print(sum)


def findIndex(list, elem):
    for index in range(len(list)):
        if list[index] == elem:
            return index
    return -1


def isPageCorrectlyOrdered(rules, update, page, currentPageIndex):
    # print("The update: ", update)
    for rule in rules:
        # print("\n\n")
        if page in rule:
            ruleParts = rule.split("|")
            indexOfRulePage = 0
            # print("Current rule: ", rule)
            # print("Current page: ", page)
            # print("Current page index: ", currentPageIndex)
            if page == ruleParts[0]:
                # print("Page is on the left, so index should be higher")
                indexOfRulePage = findIndex(update, ruleParts[1])
                # print("Index of the other part of the rule: ", indexOfRulePage)
                if indexOfRulePage == -1:
                    continue
                if indexOfRulePage > currentPageIndex:
                    # correctly ordered
                    continue
                else:
                    return False
            elif page == ruleParts[1]:
                # print("Page is on the right, so index should be lower")
                indexOfRulePage = findIndex(update, ruleParts[0])
                # print("Index of the other part of the rule: ", indexOfRulePage)
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
    print("a")


if __name__ == "__main__":
    main()
