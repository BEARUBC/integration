import sys

print("hello world")
f = open("testOutput.txt", "w")
f.write("hello world")
f.close()

# receive data from parent via arguments
data = sys.argv[1]

# process data
modified_data = data.upper()

# send modified data to stdout
print(modified_data)