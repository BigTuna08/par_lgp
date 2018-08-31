

# gets effective instructions from file as list
def extract(fname):
    with open(fname) as f:
        in_effective = False
        for line in f.readlines():
            
                if "Effective instructions" in line:
                    in_effective = True
                elif in_effective:
                    print(line.strip())

                if line == "\n":
                    in_effective = False



def print_lines(prog_lines):
    for line in prog_lines:
        print(line)
    print("\n")


# extract("results/aug2/0_0_250000_25000_0/genos/iter0-fold4.txt")

if __name__ == '__main__':
    extract("results/long/0_0_25000000_2500000_17/genos/iter0-fold0.txt")


# extract("../results/test/0_0_25000_1000_17/genos/iter0-fold2.txt")




