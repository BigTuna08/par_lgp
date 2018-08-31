


def get_met_freqs(fname):
    skips = ["<", ">"]
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
                    if len(parts) < 4:
                        continue     # QUIT
                    for op in parts[3:]:
                        if op in skips or "$" in op or "0." in op: # dont care about these
                            continue

                        if op in freqs:
                            freqs[op] += 1
                        else:
                            freqs[op] = 1
    return freqs



def print_top_n_bad(freqs, n):
    ns = list(fr.values())
    ns.sort(reverse=True)
    # print("ns = ", ns)
    for freq in ns[:n]:
        # print("freq is ", freq)
        met = None
        for k,v in freqs.items():
            # print("k, v are :", k, v )
            if v == freq:
                met = k
                break
        print(met, freq)
        freqs.pop(met)
        
        


# def get_instr_freqs(fname):
#     freqs = {}
#     with open(fname) as f:
#         in_effective = False

#         for line in f.readlines():
#             if "Effective instructions" in line:
#                 in_effective = True
#             elif in_effective:
#                 if line == "\n":
#                     in_effective = False
#                 else:
#                     line = line.strip()
#                     if line in freqs:
#                         freqs[line] += 1
#                     else:
#                         freqs[line] = 1
#     return freqs


if __name__ == '__main__':
    the_file = "../results/long/0_0_25000000_2500000_17/genos/iter0-fold0.txt"

    ### for ops
    fr = get_met_freqs(the_file)

    s = list(fr.keys())
    s.sort()
    for f in s:
        print(f, fr[f])


    print("\n\n\n")

    print_top_n_bad(fr, 10)
# ### for instrs
# ins_fr = get_instr_freqs(the_file)
# threash = 100

# for ins in ins_fr:
#     if ins_fr[ins] >= threash:
#         print(ins_fr[ins], ins)



