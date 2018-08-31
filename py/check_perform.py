import csv

DATA_FILE = 'inputs/data3.csv'
PROG_FILE = 'parts'
OUT_FILE = PROG_FILE + "-perform"

SKIPS = ["=", "add", "[True]", ">", "[False]", "<", "subt", "mult", "pdiv"]

DATASET = []
with open(DATA_FILE, newline='') as csvfile:
    r = csv.reader(csvfile, delimiter='\t', quotechar='|')
    for row in r:
        DATASET.append(row)


def get_met_inds(prog_lines):
    m_inds = {}
    for line in prog_lines:
        parts = line.split()
        for p in parts:
            try:
                v = float(p)
                # print("float v is", v)
            except:
                if "$" in p:
                    # print("reg: ", p)
                    pass
                elif p in SKIPS:
                    pass
                else:
                    # mets.append(p)
                    m_inds[p] = DATASET[0].index(p.replace("_", "."))
                    # print("met: ", p)
    return m_inds


def run_prog(prog_lines):
    m_inds = get_met_inds(prog_lines)

    correct = 0
    total = 0
    for row in DATASET[1:]:
        regs = {}
        for line in prog_lines:
            # print("line: ", line, end=" ")
            parts = line.split()
            if "$" in parts[0]:
                v1, v2 = get_vals(regs, m_inds, parts, row)
                result = None
                if parts[2] == "add":
                    result = v1 + v2
                elif parts[2] == "subt":
                    result = v1 - v2
                elif parts[2] == "mult":
                    result = v1 * v2
                elif parts[2] == "pdiv":
                    if v2 == 0:
                        result = v1
                    else:
                        result = v1/v2
                else:
                    raise "dont know what to do!"
                # print(" | result: ", result)
                regs[parts[0]] = result

            else:
                pass
                # print("skiped")

        # print(regs["$0"], row[2])
        if regs["$0"] > 0 and row[2] == "1":
            correct += 1
        elif regs["$0"] < 0 and row[2] == "0":
            correct += 1
        total += 1
        # print("\n")
    # print(correct, total)
    return correct/total




def get_vals(regs, m_inds, parts, row):
    v1, v2 = None, None
    try:
        v1 = float(parts[3])
    except:
        if "$" in parts[3]:
            v1 = regs[parts[3]]
        else:
            v1 = float(row[m_inds[parts[3]]])
    try:
        v2 = float(parts[4])
    except:
        if "$" in parts[4]:
            v2 = regs[parts[4]]
        else:
            v2 = float(row[m_inds[parts[4]]])
    return v1, v2

    # for m in mets:
    #     print("***********", m)
    #
    # for k, v in m_inds.items():
    #     print(k, v)
# def run_line(line, regs, m_inds):
#     parts = line.split()
#     if "$" in parts[0]:
#         if parts[2] == "add":
#             pass
#         elif parts[2] == "subt":
#             pass
#         elif parts[2] == "mult":
#             pass
#         elif parts[2] == "pdiv":
#             pass
#         else:
#             raise "dont know what to do!"
#     else:
#         pass
#
#     for p in parts:
#         v = None
#         try:
#             v = float(p)
#         except:
#             if "$" in p:
#                 v = regs[p]
#             elif p in SKIPS:
#                 pass
#             else:
#                 # mets.append(p)
#                 m_inds[p] = DATASET[0].index(p.replace("_", "."))
#                 # print("met: ", p)

def get_progs():
    progs = []
    with open(PROG_FILE) as f:
        new_prog = []
        for line in f.readlines():
            if line == "\n":
                if len(new_prog) > 0:
                    progs.append(new_prog)
                    new_prog = []
            else:
                new_prog.append(line.strip())

            # print(line.strip())
    return progs

if __name__ == '__main__':
    with open(OUT_FILE, "w") as f:
        ps = get_progs()

        for p in ps:
            res = run_prog(p)
            for line in p:
                print(line, file=f)
            print(res, "\n", file=f)

