import json
import sys

print("hello world")

# receive data from parent via arguments
data = sys.argv[1]

# process data
modified_data = json.loads(data)

# send modified data to stdout
print(modified_data)
f = open("test2Output.txt", "w")
f.write("before\n")
f.write(modified_data["array"])
f.write("after\n")
f.close()