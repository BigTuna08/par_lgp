from check_perform import get_progs, run_prog
from decompose_progs import decomp, simplify1, simplify2


def print_progs(progs):
    for p in progs:
        for line in p:
            print(line)
        print()


def decomp_all(progs_list, max_len = None):
    new_list = []

    for prog in progs_list:
        for sub_prog in decomp(prog):
            if max_len is None or len(sub_prog) <= max_len:
                new_list.append(sub_prog)

    return new_list



if __name__ == '__main__':
        progs = get_progs()

        sub_progs = decomp_all(progs, 4)

        print_progs(sub_progs)

