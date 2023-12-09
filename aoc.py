#!./venv/bin/python3

MSG = """
Utility to quickly create and test new projects for days

Commands:
 - new [day]    | Creates a new Rust project for the day after latest folder, if no concrete day is provided
 - test [day]   | Tests the latest day, if no concrete day is provided
 - part2 [day]  | Fetches the part2 tests for the latest day, if no concrete day is provided
 - clean [day]  | Cleans target day. If no day is specified, cleans all

"""

import subprocess
import re
import requests
import os
import sys
from aocd import get_data
from aocd.models import Puzzle
from glob import glob
from templater import fill_template

def make_test(part, file, result):
    with open(f"{TEMPLATE}/test.rs.template", "r") as f:
        template = f.read()

    return fill_template(template, _start=r"\$\$", _end=r"\$\$", part=str(part), file=str(file), result=str(result))

TOKEN = os.getenv('AOC_SESSION')
YEAR=2023

TEMPLATE = "rs_template"
days = glob("day*")
day = int(max([re.search(r'\d+', day).group() for day in days]))
day_str = "day" + str(day).zfill(2)

if len(sys.argv) <= 1 or "--help" in sys.argv:
    print(MSG)
    exit()

if sys.argv[1] == "new":
    if len(sys.argv) == 3:
        day = int(sys.argv[2])
    else:
        day += 1

    day_str = "day" + str(day).zfill(2)

    r = requests.get(f"https://adventofcode.com/{YEAR}/day/{day}")
    if r.status_code == 404:
        print(f"Day {day} not available yet")
        exit()

    files = glob(f"{TEMPLATE}/*.rs")

    subprocess.run(["cargo", "new", "--lib", day_str])
    subprocess.run(["cargo", "add", "regex"], cwd=day_str)

    for file in files:
        subprocess.run(["cp", file, f"{day_str}/src/"])

    with open(f"{day_str}/.gitignore", "w") as f:
        f.writelines(["target/\n",
                    "inputs/prompt.txt\n"
                      ])


    # Get Tests
    subprocess.run(["mkdir", f"{day_str}/inputs"])

    puzzle = Puzzle(year=YEAR, day=day)
    for i, example in enumerate(puzzle.examples):
        test_file = f"input{i+1}"
        data = example.input_data
        answer = example.answer_a

        if answer is None:
            continue

        with open(f"{day_str}/inputs/{test_file}.txt", "w") as f:
            f.write(data)

        with open(f"{day_str}/src/tests.rs", "a") as f:
            f.write("\n")
            f.write(make_test(1, test_file, answer))

    with open(f"{day_str}/inputs/prompt.txt", "w") as f:
        f.write(str(get_data(day=day, year=YEAR)))

elif sys.argv[1] == "part2":
    if len(sys.argv) == 3:
        day = int(sys.argv[2])
        day_str = "day" + str(day).zfill(2)

    puzzle = Puzzle(year=YEAR, day=day)
    for i, example in enumerate(puzzle.examples):
        test_file = f"input{i+1}"
        data = example.input_data
        answer = example.answer_b

        if answer is None:
            continue

        with open(f"{day_str}/inputs/{test_file}.txt", "w") as f:
            f.write(data)

        with open(f"{day_str}/src/tests.rs", "a") as f:
            f.write("\n")
            f.write(make_test(2, test_file, answer))

elif sys.argv[1] == "test":
    l = 2
    if len(sys.argv) >= 3:
        try:
            day = int(sys.argv[2])
            day_str = "day" + str(day).zfill(2)
            l += 1
        except:
            pass

    RELEASE = ""
    if len(sys.argv) == l + 1:
        RELEASE = "--" + sys.argv[l]

    subprocess.run(["cargo", "test", RELEASE], cwd=day_str)

elif sys.argv[1] == "test-all":
    all_days = sorted(glob("day*"))
    blacklist = []
    if len(sys.argv) >= 3 and sys.argv[2] == "--skip":
        blacklist = list(map(lambda x: int(x), sys.argv[3:]))

    for i, day in enumerate(all_days):
        if i + 1 in blacklist:
            continue
        subprocess.run(["cargo", "test", "--release"], cwd=day)

elif sys.argv[1] == "clean":
    days = sorted(glob("day*"))
    l = 2
    if len(sys.argv) >= 3:
        try:
            day = int(sys.argv[2])
            days = ["day" + str(day).zfill(2)]
            l += 1
        except:
            pass

    for i, day in enumerate(days):
        subprocess.run(["cargo", "clean"], cwd=day)

