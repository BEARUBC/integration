import json
import sys

print("hello world")

# receive data from parent via arguments
data = json.dumps({'array': [1, 2]})

# process data
modified_data = json.loads(data)

# send modified data to stdout
print(modified_data)
f = open("test2Output.txt", "w")
f.write("before\n")
for i in modified_data["array"]:
	f.write((i))
f.write("after\n")
f.close()