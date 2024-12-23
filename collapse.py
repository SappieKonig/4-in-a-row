import glob

res = "This is all the code in the project.\n\n"
for file in glob.glob("src/**/*.rs"):
    res += f'{file}\n'
    with open(file, "r") as f:
        content = f.read()
    res += content
    res += '\n'

    res += '-' * 100
    res += '\n'

with open("prompt.tmp", "w") as f:
    f.write(res)
