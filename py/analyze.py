

# gets effective instructions from file as list
def get_prog_lines(fname, target):
    target_lines = []
    with open(fname) as f:
        found_target = False
        in_effective = False
        for line in f.readlines():
            if target in line:
                found_target = True
            elif found_target:
                if "Effective instructions" in line:
                    in_effective = True
                elif in_effective:
                    if line == "\n":
                        break
                    else:
                        target_lines.append(line.strip())
    return target_lines


# not used- divides instructions to 2 types
def split_branches(prog_lines):
    branches = []
    non_branches = []
    for line in prog_lines:
        # print("line is: ", line)
        if "Skip" in line:
            branches.append(line)
        else:
            non_branches.append(line)
    return {"br":branches, "non_br":non_branches}


# returns list of instruction indexes which might be skipped
def find_br_points(prog_lines):
    last_br = False
    br_points = []
    i = 0
    for line in prog_lines:
        if "Skip" in line:
            last_br = True
        else:
            if last_br is True:
                br_points.append(i)
            last_br = False
        i += 1
    return br_points


# like main, just read and print some stuff
def anaylze(fname, target):
    t_lines = get_prog_lines(fname, target)
    # spl = split_branches(t_lines)
    # print(spl)

    for aline in t_lines:
        print(aline)

    br = break_into_parts(t_lines)
    print(br)


# split branchining program into individual models
def break_into_parts(prog_lines):
    br_points = find_br_points(prog_lines)
    include_brs = [False for _ in br_points]

    options = []
    while True:
        # print(include_brs, "\n\n")
        in_lines = []

        for i in range(len(prog_lines)):
            if i in br_points:
                if not include_brs[br_points.index(i)]:
                    continue
            if "Skip" not in prog_lines[i]:
                in_lines.append(i)
        options.append(in_lines)


        j = len(br_points) -1
        while include_brs[j]: # is true
            j -= 1
            if j < 0:
                break
        if j < 0:
            break
        while j < len(include_brs):
            include_brs[j] = not include_brs[j]
            j += 1


    options = [get_effective_instruction_i(prog_lines, model_lines) for model_lines in options]
    print("\n")
    seen = []
    unique = []
    for i in range(len(options)):
        if i not in seen:
            unique.append(i)
        for j in range(i+1, len(options)):
            if options[j] == options[i]:
                seen.append(j)


    unique = [options[i] for i in unique]
    for unique_model in unique:

        prog = [prog_lines[l] for l in unique_model]

        print_lines(prog)
        # # print("** After **")
        # print_lines(get_effective_instructions(prog))
        # print("\n\n")



def get_effective_instructions(prog_lines):
    eff_instrs = []
    eff_regs = set()
    return_reg = "$0"
    eff_regs.add(return_reg)

    for i in range(len(prog_lines)-1,0, -1):
        parts = prog_lines[i].split()
        if parts[0] in eff_regs:
            eff_regs.remove(parts[0])
            eff_regs.add(parts[3])
            eff_regs.add(parts[4])
            eff_instrs.insert(0,prog_lines[i])

    return eff_instrs


def get_effective_instruction_i(all_prog_lines, model_lines):
    # print("\n\n\n")
    # print("model lines are ", model_lines)
    eff_instrs_i = []
    eff_regs = set()
    return_reg = "$0"
    eff_regs.add(return_reg)

    for i in range(len(all_prog_lines) - 1, -1, -1):
        if i not in model_lines: continue

        parts = all_prog_lines[i].split()
        # print("parts are ", parts)
        if parts[0] in eff_regs:
            eff_regs.remove(parts[0])
            eff_regs.add(parts[3])
            eff_regs.add(parts[4])
            eff_instrs_i.insert(0, i)

    # print("eff lines are ", eff_instrs_i)


    return eff_instrs_i


def print_lines(prog_lines):
    for line in prog_lines:
        print(line)
    print("\n")

anaylze("../results/5iterlong/0_0_10000000_500000_0/genos/iter0-fold4.txt", "(4,10)")






