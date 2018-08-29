r = 20
c = 32

print("[", end = "")
for i in range(r):
    print("[", end = "")
    for j in range(c-1):
        print("None", end = ", ")
    print("None],")
print("]", end = "")
