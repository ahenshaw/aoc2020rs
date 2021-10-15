import argparse
import os.path
import sys

year = "2020"

parser = argparse.ArgumentParser()
parser.add_argument('day')
args = parser.parse_args()

fn = f"src/day_{args.day}.rs"
if os.path.exists(fn):
    response = input("File already exists. Overwrite? [Y/n] ")
    if response and not response.lower().startswith('y'):
        print("Cancelling")
        sys.exit()
print(f"Writing to {fn}")
template = open('src/template.rs').read()
output = template.replace("day#", f"day{args.day}")
with open(fn, 'w') as fh:
    fh.write(output)

libname = "src/lib.rs"
lib = open(libname).read()
to_insert = f"pub mod day_{args.day};\n"
if to_insert not in lib:
    i = lib.find("// end modules")
    output = lib[:i] + to_insert + lib[i:]
    with open(libname, 'w') as fh:
        fh.write(output)

# create a blank test data file
fn = f"input/{year}/test{args.day}.txt"
if not os.path.exists(fn):
    open(fn, "w")
