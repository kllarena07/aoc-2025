import time
start = time.time()
in_file = open("test.in", "r")

lines = in_file.readlines()

def check_line(j, num_digits, repeated, string_i): # j is length of str
  if (num_digits % len(repeated) != 0): # if u cant evenly divide in, it won't work
    return False
  
  for k in range(j, num_digits + 1, j): # iterate from end of repeated string to end, stepping by j
    check_str = string_i[k-j:k]
    
    if (repeated != check_str):
      return False
  
  return True

def check_string(i): 
  string_i = str(i)
  num_digits = len(string_i) 

  half_digits = int(num_digits / 2)
  for j in range(1, half_digits + 1): # length of repeated
    repeated = string_i[:j] 
    
    if check_line(j, num_digits, repeated, string_i):
      return True
  
  return False
      

def calculate_invalid(start, begin):
  total = 0

  for i in range(start, begin + 1):
    if check_string(i):
      total += i

  return total

result = 0
for line in lines:
  line = line.strip(',\n').split(",")
  for r in line:
    rge = r.split("-")
    result += calculate_invalid(int(rge[0]), int(rge[1]))

print(result)
print("time:", time.time() - start)
