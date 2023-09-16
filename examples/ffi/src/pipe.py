import json

print("-------------- PYTHON --------------")
with open('examples/demo/src/pipe.json', 'r') as f:
  data = json.load(f)

print("(pipe.py)", data["data"])
print("------------ END PYTHON ------------")


