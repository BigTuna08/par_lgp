test_prog = [
"$6 =   add -0.33333334 Arg",
"$4  =   add 0.16666667  SM_C16_1",
"$0  =   subt    -0.33333334 Ac_Orn",
"Skip next if $4 >   C18",
"$6  =   add 0.016393442 PC_aa_C42_6",
"Skip next if $6 >   $0",
"$0  =   add Orn -0.06666667",
"Skip next if Arg    >   $0",
"$0  =   mult    $0  $0"
]



def decomp(prog_lines):
    branch_i = []
    for i, line in enumerate(prog_lines):
        if "Skip" in line:
            branch_i.append(i)

    n_branches = len(branch_i)

    sub_progs = []

    for run_i in range(2 ** n_branches):

        sub_prog = []
        skip_next = False
        for instr_i, instr in enumerate(prog_lines):
            if "Skip" in instr:
                skip_next = check_skip(instr_i, run_i, branch_i)
                s = instr.replace("Skip next if", "[{}]".format(skip_next))
                sub_prog.append(s)
                # print(skip_next, end = "\t")
            elif skip_next:
                skip_next = False
            elif "QUIT" in instr:
                break
            else:
                # print("appending ", instr)
                sub_prog.append(instr)

        # print("\ndone prog\n")
        sub_progs.append(sub_prog)

    return sub_progs


# True    True    True    
# False   True    True    
# True    False   True    
# False   False   True    
# True    True    False   
# False   True    False   
# True    False   False   
# False   False   False   
def check_skip(instr_i, run_i, branch_i):
    i = branch_i.index(instr_i)
    v = run_i  // 2 ** i
    result = v % 2 == 0
    # print(i, v, result)
    return result


def simplify1(prog_lines):
    new_lines = []
    assignments = {}

    for line in prog_lines:
        parts = line.split()
        # print("parts", parts)
        if len(parts) < 2:
            continue

        src_regs = []

        for p in parts[1:]:
            if "$" in p:
                src_regs.append(p)

        for sr in src_regs:
            line = line.replace(sr, assignments[sr])

        if "$" in parts[0]:  #assignment
            assignments[parts[0]] = "{}({}, {})".format(parts[2], parts[3], parts[4])

        new_lines.append(line)

    return new_lines


def simplify2(prog_lines):
    new_lines = []
    assignments = {}
    conditions = []

    last_r0 = None
    for line in prog_lines:
        parts = line.split()
        # print("parts", parts)
        if len(parts) < 2:
            continue

        src_regs = []

        for p in parts[1:]:
            if "$" in p:
                src_regs.append(p)


        to_replace = line.split("=")[-1]
        for sr in src_regs:
            to_replace = to_replace.replace(sr, assignments[sr])

        if "$" in parts[0]:  #assignment
            assignments[parts[0]] = "{}({}, {})".format(parts[2], parts[3], parts[4])
            to_replace = parts[0] + " = " + to_replace
        else:
            conditions.append(to_replace)

        if parts[0] == "$0":
            last_r0 = to_replace

        new_lines.append(to_replace)



# extract("results/aug2/0_0_250000_25000_0/genos/iter0-fold4.txt")


if __name__ == '__main__':
    subs = decomp(test_prog)

    with open("../parts", 'w') as out_f:
        for sp in subs:
            print("\n", file=out_f)
            for line in sp:
                print(line, file=out_f)

    # print("\nsimp \n")
    # for line in simplify1(sp):
    #     print(line)

    # simplify2(sp)

# extract("../results/test/0_0_25000_1000_17/genos/iter0-fold2.txt")




