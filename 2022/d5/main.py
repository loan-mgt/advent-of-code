
with open('test.txt') as f:
    lines = f.readlines()

piles = [[] for i in range(len(lines[0]))]

print(piles)
split = 0
for c,line in enumerate(lines):
    if line[0] == '\n':
        split = c
        break
print(split)