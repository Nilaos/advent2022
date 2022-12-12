sum = 0

for ln in open("day3/input3.txt"):
    ln = ln.strip()

    first, second = ln[: len(ln) // 2], ln[len(ln) // 2 :]

    ignore = []

    for part in first:
        if part in ignore:
            continue
        if second.find(part) != -1:
            if part.isupper():
                sum += ord(part) - ord("A") + 1 + 26
            else:
                sum += ord(part) - ord("a") + 1
            ignore.append(part)

print(sum)


with open("day3/input3.txt") as f:
    lines = f.readlines()
    sum = 0

    i = 0
    while i < len(lines):
        first, second, third = (
            lines[i].strip(),
            lines[i + 1].strip(),
            lines[i + 2].strip(),
        )

        for part in first:
            if second.find(part) != -1 and third.find(part) != -1:
                if part.isupper():
                    sum += ord(part) - ord("A") + 1 + 26
                else:
                    sum += ord(part) - ord("a") + 1
                break
        i += 3

    print(sum)
