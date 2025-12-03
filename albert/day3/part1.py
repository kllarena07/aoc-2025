in_file = open("test.in", "r")

lines = in_file.readlines()

total = 0

for line in lines:
  line = line.strip(' \n')
  m = max(line)
  m_i = line.find(m)
  
  if m_i == len(line) - 1: # last element, special case
    new_line = line[:-1] # ignore the last one
    m = max(new_line)
    m_i = new_line.find(m)

  n = line[m_i + 1]
  for j in range(m_i + 1, len(line)):
    if line[j] > n:
      n = line[j]
      
  total += int(m+n)

print(total)

  