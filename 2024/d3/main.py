import re

regex = r'mul\((?P<left>[0-9]{1,3}),(?P<right>[0-9]{1,3})\)'

f = open("in.txt", "r")
raw = f.read()

matches = re.finditer(regex, raw)

res = 0

for match in matches:
    print(match)
    l, r = int(match.group('left')), int(match.group('right'))
    res += l * r

print(res)