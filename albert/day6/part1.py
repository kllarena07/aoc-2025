
in_file = open("test.in", "r")

lines = in_file.readlines()
lines = [line.strip().split() for line in lines]

total = 0
for i in range(len(lines[0])):
  if lines[-1][i] == "+":
    result = 0
    for j in range(len(lines) - 1):
      result += int(lines[j][i])

    total += result
  else:
    result = 1
    for j in range(len(lines) - 1):
      result *= int(lines[j][i])

    total += result
  
print(total)