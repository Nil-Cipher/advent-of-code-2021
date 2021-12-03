count = 0
with open('input') as f:
    inp = f.readlines()

for i in range(4, len(inp) + 1):
    if sum(map(int, inp[i-3:i])) > sum(map(int, inp[i - 4:i-1])):
        count +=1
print(count)