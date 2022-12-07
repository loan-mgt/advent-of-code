class Dir:
    def __init__(self, name):
        self.name = name
        self.ls = []
   
        
        

    def __repr__(self):
        #return "- "+self.name
        return "- "+self.name+" (dir) \n    "+str(self.ls)
    

class File:
    def __init__(self,name,size):
        self.name = name
        self.size = size
    def __repr__(self):
        return "[FILE] "+self.name+" :"+self.size
    


pile = []
with open('test.txt') as f:
    lines = f.readlines()


for line in lines:
    print(line)
    if line[0] == "$":

        if line[:4] == "$ cd":
            if pile == []:
                tmp = Dir(line[5:-1])
                pile.append(tmp)
                print("add Dir ", line[5:-1])

            
            elif line[5:-1] == ".." and len(pile)>1 :
                pile.pop()
                print("popoed", pile)
            
            elif line[5:-1] not in [i.name for i in pile[-1].ls]:
                tmp = Dir(line[5:-1])
                
                pile[-1].ls.append(tmp)
                print("debug",pile[-1].ls)
                print("debug",tmp.ls)
                pile.append(tmp)
                print("add Dir ", line[5:-1])


print(pile)


print(pile[0])
