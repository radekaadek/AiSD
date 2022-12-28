import subprocess
import re
import os


## This script compiles and runs all .cpp and .h files in the current directory 
## It compares the output with the .out files in the tests directory.
## It also runs the program with gprof, ccpcheck and valgrind
## and saves the output in the performance, ccpcheck and valgrind directories.
## example input is in the rank directory


def check_for_dependencies() -> bool:
    # Check for g++
    if subprocess.run("which g++", shell=True, capture_output=True).returncode != 0:
        print("G++ not found, run 'sudo apt install g++' to install it")
        return False
    
    # gprof
    if subprocess.run("which gprof", shell=True, capture_output=True).returncode != 0:
        print("Gprof not found, run 'sudo apt install gprof' to install it")
        return False

    # valgrind
    if subprocess.run("which valgrind", shell=True, capture_output=True).returncode != 0:
        print("Valgrind not found, run 'sudo apt install valgrind' to install it")
        return False

    # cppcheck
    if subprocess.run("which cppcheck", shell=True, capture_output=True).returncode != 0:
        print("Cppcheck not found, run 'sudo apt install cppcheck' to install it")
        return False
    
    return True

def get_source_files() -> list:
    # Check for cpp and h files in the current directory
    cpp_files = [f for f in os.listdir(".") if re.match(r".*\.cpp", f)]
    h_files = [f for f in os.listdir(".") if re.match(r".*\.h", f)]
    if not cpp_files and not h_files:
        return []
    return cpp_files

def get_tests() -> list:
    # Get all .in files from the tests directory
    if os.path.isdir("tests"):
        test_files = [f for f in os.listdir("tests") if re.match(r".*\.in", f)]
        if not test_files:
            return []
        return test_files
    return []

def run_tests() -> None:
    # Check for dependencies
    if not check_for_dependencies():
        return

    # Get all source files
    cpp_files = get_source_files()
    if not cpp_files:
        print("No cpp or h files found in the current directory")
        return

    # Get all test files
    test_files = get_tests()
    if not test_files:
        print("No tests found in the tests directory")
        return
    test_files.sort()

    # Get all .out files from the tests directory
    output_files = [f for f in os.listdir("tests") if re.match(r".*\.out", f)]
    output_files_set = set(output_files)

    # Run cppcheck
    print(f"Running cppcheck...")
    with open("tests/cppcheck.txt", "w") as f:
        subprocess.run(f"cppcheck {' '.join(cpp_files)}", shell=True, stdout=f, stderr=f)

    # Get all .cpp files and compile them into a single executable
    print(f"Compiling...")
    subprocess.run(f"g++ -std=c++2a -Wall -Wextra -Werror -Wextra -pedantic -O3 -o project.exe {' '.join(cpp_files)}", shell=True)
    # performance test
    subprocess.run(f"g++ -std=c++2a -pedantic -O3 -pg -o performance.exe {' '.join(cpp_files)}", shell=True)
    # valgrind
    subprocess.run(f"g++ -std=c++2a -pedantic -ggdb3 -O3 -o valgrind.exe {' '.join(cpp_files)}", shell=True)

    # Make directories for performance, valgrind and results
    subprocess.run("mkdir -p tests/valgrind", shell=True)
    subprocess.run("mkdir -p tests/performance", shell=True)
    subprocess.run("mkdir -p tests/output", shell=True)
    

    # Run each test
    for test in test_files:
        test_name = test.split(".")[0]
        print(f"Running test {test_name}")

        # Run the project
        subprocess.run(f"./project.exe < tests/{test_name}.in > tests/output/{test_name}.out", shell=True)

        # Check if the output file exists for comparison
        if f"{test_name}.out" in output_files_set:
            # Compare the output with the expected output file
            with open(f"tests/{test_name}.out", "r") as f:
                expected_output = f.read()
            with open(f"tests/output/{test_name}.out", "r") as f:
                actual_output = f.read()
            if expected_output != actual_output:
                print(f"Test {test_name} failed")
                print(f"Expected: {expected_output}")
                print(f"Got: {actual_output}")

        # Run the gprof test
        subprocess.run(f"./performance.exe < tests/{test_name}.in > /dev/null", shell=True)
        subprocess.run(f"gprof performance.exe gmon.out > tests/performance/{test_name}.txt", shell=True)

        # Run the valgrind test
        subprocess.run(f"valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes --log-file=tests/valgrind/{test_name}.txt ./valgrind.exe < tests/{test_name}.in > /dev/null", shell=True)
    
    # Clean up
    subprocess.run("rm *.exe", shell=True)
    subprocess.run("rm gmon.out", shell=True)


if __name__ == "__main__":
    run_tests()
