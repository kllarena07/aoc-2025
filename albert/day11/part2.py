in_file = open("test.in", "r")

lines = in_file.readlines()

routes = {}

links = [line.strip().split(":") for line in lines]

mem = {}

for l in links:
  routes[l[0]] = l[1].strip().split(" ")
  
def calculate_routes(start, node, target):
  if (start, node) in mem:
    return mem[(start, node)]
    
  if node == "out" and target != "out":
    return 0
  
  if node == target:
    return 1
  
  total = 0
  for r in routes[node]:
    total += calculate_routes(start, r, target)
  
  mem[(start, node)] = total
  return total

svrfft = calculate_routes("svr", "svr", "fft")
fftdac = calculate_routes("fft", "fft", "dac")
dacout = calculate_routes("dac", "dac", "out")

print(svrfft, fftdac, dacout)

print(svrfft * fftdac * dacout)