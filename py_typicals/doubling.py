'''
    [doubling]
    https://algo-logic.info/doubling/
    https://atcoder.jp/contests/abc167/tasks/abc167_d
    https://atcoder.jp/contests/abc179/tasks/abc179_e
    https://atcoder.jp/contests/abc013/tasks/abc013_4
'''

from math import log2

n,k = map(int, input().split())
al = list(map(int, input().split()))

logk = int(log2(k))+1
db = [ [0]*n for _ in range(logk) ]
for ni in range(n): 
    db[0][ni] = al[ni]-1
for ki in range(logk-1):
    for ni in range(n):
        db[ki+1][ni] = db[ki][db[ki][ni]]

now = 0
for i in range(logk):
    if k&(1<<i) > 0:
        now = db[i][now]
print(now+1)


# ----------------------------------

# https://atcoder.jp/contests/abc179/tasks/abc179_e
from math import log2
n,x,m = map(int, input().split())

logn = int(log2(n))+1
db = [[0]*m for _ in range(logn)]
dbs = [[0]*m for _ in range(logn)]

# a=i なら 次は (i**2)%m
for i in range(m):
    db[0][i] = (i**2)%m
    dbs[0][i] = (i**2)%m

for i in range(logn-1):
    for j in range(m):
        db[i+1][j] = db[i][db[i][j]]
        dbs[i+1][j] = dbs[i][db[i][j]] + dbs[i][j]

ans = x
now = x
for i in range(logn):
    if (n-1)&(1<<i) > 0:
        ans += dbs[i][now]
        now = db[i][now]

print(ans)
