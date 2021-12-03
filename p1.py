

lines = []
with open('input') as f:
    lines = f.readlines()
nums = [int(x) for x in lines]
s = float('inf')
count = 0

for i in range(2, len(nums)):
    low = i-2
    # print(low, i)
    if sum(nums[low: i+1]) > s:
        count += 1
    s = sum(nums[low: i+1])
    # if(i < 5):
    # print(s)
print(count)
