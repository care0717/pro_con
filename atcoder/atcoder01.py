N = input()
L = map(int, raw_input().split())
L.sort()
res = 0
for i in range(0, N):
    res += L[2*n]
print(res)
