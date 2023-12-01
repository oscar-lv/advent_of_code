input = open("day1_input.txt").read()


# Part 1
def count(input):
    count = 0
    for c in input:
        if c == "(":
            count += 1
        else:
            count -= 1
    return count


# Part 2
def count_to_basement(input):
    count = 0
    for i, c in enumerate(input):
        if c == "(":
            count += 1
        else:
            count -= 1
        if count == -1:
            return i + 1


print("Part 1:", count(input))
print("Part 2:", count_to_basement(input))
