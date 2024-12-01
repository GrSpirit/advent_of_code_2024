import subprocess
import os

for d in filter(lambda f: f.startswith("day"), os.listdir()):
    print(f"test {d}")
    os.chdir(d)
    subprocess.run(["cargo", "test", "--verbose"])
    os.chdir("..")
