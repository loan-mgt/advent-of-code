




def get_cal(lines):
    res = []
    cal = 0
    for i in lines:
        if i == "\n":
            res.append(cal)
            cal = 0
        else:
            cal += int(i[:-1])

    return res


if __name__=="__main__":

    with open('main.txt') as f:
        lines = f.readlines()

    res = get_cal(lines)
    #print(res)

    print("part 1: ",max(res))

    ## part 2

    res.sort()

    print(res[-3:])

    print("part 2: ",sum(res[-3:]))