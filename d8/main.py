class Tree:
    def __init__(self, height):
        self.visible = True
        self.height = height
        # part 2
        self.score = 0

    def __repr__(self) -> str:
        return (
            "<"
            + ("V" if self.visible == True else "H")
            + str(self.height)
            + "s"
            + str(self.score)
            + ">"
        )


# read file
with open("main.txt") as f:
    lines = f.readlines()


for i in range(len(lines) - 1):
    lines[i] = lines[i][:-1]


tree_map = [
    [Tree(lines[z][w]) for w in range(len(lines[z]))] for z in range(len(lines))
]


def check_visibile_tree(tree_map):
    for i in range(1, len(tree_map) - 1):
        for z in range(1, len(tree_map[i])):
            # looking from the left
            left = True
            for s in range(0, z):
                # print("left i,z,s",i,z,s,tree_map[i][s].height > tree_map[i][z].height,tree_map[i][s].height , tree_map[i][z].height)
                if tree_map[i][s].height >= tree_map[i][z].height:
                    left = False
                    break
            # looking from the right
            right = True
            for s in range(z + 1, len(lines[i])):
                # print("right i,z,s",i,z,s,tree_map[i][s].height > tree_map[i][z].height,tree_map[i][s].height , tree_map[i][z].height)
                if tree_map[i][s].height >= tree_map[i][z].height:
                    right = False
                    break
            # looking from the up
            up = True
            for s in range(0, i):
                # print("up i,z,s",i,z,s,tree_map[s][z].height >= tree_map[i][z].height,tree_map[s][z].height , tree_map[i][z].height)
                if tree_map[s][z].height >= tree_map[i][z].height:
                    up = False
                    break
            # looking from the down
            down = True
            for s in range(i + 1, len(lines)):
                # print("down i,z,s",i,z,s,tree_map[s][z].height >= tree_map[i][z].height,tree_map[s][z].height , tree_map[i][z].height)
                if tree_map[s][z].height >= tree_map[i][z].height:
                    down = False
                    break

            # print(left,right,up,down)
            tree_map[i][z].visible = False if right + left + up + down == 0 else True


def nb_visibile(tree_map):
    count = 0
    for i in range(1, len(tree_map) - 1):
        for z in range(1, len(tree_map[i]) - 1):
            if tree_map[i][z].visible:
                count += 1
    return count + (len(tree_map) * 2) + ((len(tree_map[0]) - 2) * 2)


check_visibile_tree(tree_map)
print(nb_visibile(tree_map))


## part 2


def check_score_tree(tree_map):
    for i in range(1, len(tree_map) - 1):
        for z in range(1, len(tree_map[i]) - 1):

            # looking from the left
            left = 1
            for s in range(z - 1, -1, -1):
                left = z - s
                if tree_map[i][s].height >= tree_map[i][z].height:
                    break

            # looking from the right
            right = 1
            for s in range(z + 1, len(lines[i])):
                right = s - z
                if tree_map[i][s].height >= tree_map[i][z].height:
                    break

            # looking from the up
            up = 1
            for s in range(i - 1, -1, -1):
                up = i - s
                if tree_map[s][z].height >= tree_map[i][z].height:
                    break

            # looking from the down
            down = 1
            for s in range(i + 1, len(lines)):
                down = s - i
                if tree_map[s][z].height >= tree_map[i][z].height:
                    break

            tree_map[i][z].score = right * left * up * down


def best_score(tree_map):
    score = 0
    for i in range(1, len(tree_map) - 1):
        for z in range(1, len(tree_map[i]) - 1):
            if tree_map[i][z].score > score:
                score = tree_map[i][z].score
    return score


check_score_tree(tree_map)
print(best_score(tree_map))
