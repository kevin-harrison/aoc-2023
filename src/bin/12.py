from functools import cache

@cache
def recurse(lava, springs, result=0):
    if not springs:
        return '#' not in lava
    current, springs = springs[0], springs[1:]
    for i in range(len(lava) - sum(springs) - len(springs) - current + 1):
        if "#" in lava[:i]:
            break
        if (nxt := i + current) <= len(lava) and '.' not in lava[i : nxt] and lava[nxt : nxt + 1] != "#":
            result += recurse(lava[nxt + 1:], springs)
    return result


with open("data/inputs/12.txt", "r") as file:
    data = [x.split() for x in file.read().splitlines()]
    p1, p2 = 0, 0
    for e, (lava, springs) in enumerate(data):
        a = recurse(lava, (springs := tuple(map(int, springs.split(",")))))
        print("line", e+1, ":", a)
        p1 += a
        # p2 += recurse("?".join([lava] * 5), springs * 5)
    print(p1, p2)