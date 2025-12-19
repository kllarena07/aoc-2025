in_file = open("test.in", "r")

lines = in_file.readlines()

routes = {}

links = [line.strip().split(":") for line in lines]

for l in links:
  routes[l[0]] = l[1].strip().split(" ")
  
def calculate_routes(node):
  if node == "out":
    return 1
  
  total = 0
  for r in routes[node]:
    total += calculate_routes(r)
  
  return total

print(calculate_routes("you"))