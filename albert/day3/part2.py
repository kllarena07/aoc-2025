in_file = open("day3/test.in", "r")

lines = in_file.readlines()

total = 0
NUM_DIGITS = 12

for line in lines:
  current_str = ""
  line = line.strip('\n')
  
  for i in range(len(line)):
    if current_str == "": # first case
      current_str += line[i]
      continue
      
    chars_needed = NUM_DIGITS - len(current_str)
    chars_left = len(line) - i
    end_char = current_str[-1]
    
    if chars_left == chars_needed: # add the rest
      current_str += line[i:]
      break
    
    if line[i] > end_char: # we replace a char
      cur = len(current_str) - 1 # pointer to end of current_str
      moved = 0 # how much we've moved to the left
      while (int(current_str[cur]) < int(line[i]) and chars_needed + moved < chars_left and cur >= 0):

        cur -= 1
        moved += 1
      
      # replace the char
      if cur == -1: # if we're at the beginning, we just set it to be the only char
        current_str = line[i]
      else: # otherwise we just add it to the end
        current_str = current_str[:cur+1] + line[i]
      
    elif chars_needed > 0: # append to the end if we still have chars left
      current_str += line[i]
      
  total += int(current_str)

print(total)

  