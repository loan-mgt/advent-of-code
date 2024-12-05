f = open("in.txt", "r")
raw = f.read()

left = []

right = []

for l in raw.split('\n'):
    x, y = l.split('   ')
    left.append(x)
    right.append(y)

left.sort()

right.sort()

delta = 0

for i in range(len(left)):
    delta += abs(int(left[i]) - int(right[i]))

print(delta)


sim = 0
for i in range(len(left)):
    sim += int(left[i]) *   right.count(left[i])


print(sim)