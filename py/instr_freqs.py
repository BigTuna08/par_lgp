
# gets # of time each op appears in file
def get_op_freqs(fname):
    freqs = {}
    with open(fname) as f:
        in_effective = False

        for line in f.readlines():
            if "Effective instructions" in line:
                in_effective = True
            elif in_effective:
                if line == "\n":
                    in_effective = False
                else:
                    parts = line.split()
                    op = parts[0] # incase QUIT
                    if len(parts) > 2:
                        op = parts[2]
                    if op in freqs:
                        freqs[op] += 1
                    else:
                        freqs[op] = 1
    return freqs


# gets # of times each full instruction occurs 
def get_instr_freqs(fname):
    freqs = {}
    with open(fname) as f:
        in_effective = False

        for line in f.readlines():
            if "Effective instructions" in line:
                in_effective = True
            elif in_effective:
                if line == "\n":
                    in_effective = False
                else:
                    line = line.strip()
                    if line in freqs:
                        freqs[line] += 1
                    else:
                        freqs[line] = 1
    return freqs




if __name__ == '__main__':
    the_file = "../results/long/0_0_25000000_2500000_17/genos/iter0-fold0.txt"

    ### for ops
    fr = get_op_freqs(the_file)
    sum = 0
    for f in fr:
        sum += fr[f]
    s = list(fr.keys())
    s.sort()
    for f in s:
        print(f, '{0:.0f}'.format(fr[f]*100/sum)) # as proportion of total


    print("\n\n\n")

    ### for instrs
    ins_fr = get_instr_freqs(the_file)
    threash = 100

    for ins in ins_fr:
        if ins_fr[ins] >= threash:
            print(ins_fr[ins], ins)



