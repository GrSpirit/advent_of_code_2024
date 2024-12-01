import subprocess
import os

for d in filter(lambda f: f.startswith("day"), os.listdir()):
    print(f"build {d}")
    os.chdir(d)
    subprocess.run(["cargo", "build", "--verbose"])
    os.chdir("..")
