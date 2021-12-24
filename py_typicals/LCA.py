'''
    [LCA (with doubling)]
    https://atcoder.jp/contests/abc014/tasks/abc014_4
    https://algo-logic.info/lca/
'''

from collections import deque
from math import log2

n = int(input())
g = [ [] for _ in range(n) ]
for _ in range(n-1):
    x,y = map(int, input().split())
    x-=1
    y-=1
    g[x].append(y)
    g[y].append(x)


# --- calc (dist-from-root, parent) of all the nodes ---
q = deque([0])
dist_pares = [(-1,-1)]*n
dist_pares[0] = (0,0)
while q:
    poped = q.popleft()
    dist = dist_pares[poped][0]
    pare_node = dist_pares[poped][1]
    for next_node in g[poped]:
        if next_node != pare_node:
            q.append(next_node)
            dist_pares[next_node] = (dist+1, poped)
# print(dist_pares)

# --- doubling (for calulating the parent) ---
logn = int(log2(n))+1
db = [ [0]*n for _ in range(logn) ]
for ni in range(n): 
    db[0][ni] = dist_pares[ni][1]
for ki in range(logn-1):
    for ni in range(n):
        db[ki+1][ni] = db[ki][db[ki][ni]]
# print(db)


ansl = []
Q = int(input())
for _ in range(Q):
    a,b = map(int, input().split())
    a-=1
    b-=1
    dist_a = dist_pares[a][0]
    dist_b = dist_pares[b][0]
    if dist_a > dist_b:
        dist_a,dist_b = dist_b,dist_a
        a,b = b,a

    # --- for depth-a == depth-b ---
    moved_a = a
    moved_b = b
    b_up = dist_b-dist_a 
    for i in range(logn):
        if b_up&(1<<i) > 0:
            moved_b = db[i][moved_b]
    
    # --- search LCA ---
    dist = b_up
    for i in range(logn-1,-1,-1):
        if db[i][moved_a] != db[i][moved_b]:
            moved_a = db[i][moved_a]
            moved_b = db[i][moved_b]
            dist += 2*pow(2,i)
    if moved_a != moved_b: dist += 2
    ans = dist+1
    ansl.append(ans)
    # lca = db[0][moved_b]

for a in ansl: print(a)