"""
bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11
"""



datastream = "bvwbjplbgvbhsrlpgdmjqwftvncz"


def getFirstMarker(ds):
    for i in range(0,len(ds)-4):
        found = False
        for z in ds[i:i+4]:
            if not found and ds[i:i+4].count(ds[i:i+4].replace(z,"",1)):
                print("found double of , in",z,ds[i:i+4] )
                found = True
        if not found:
            return i+4
        


getFirstMarker(datastream)