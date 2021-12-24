# not-recursive rerooting

from collections import deque
def make_order_parents(n,gl,root=0):
    order=[root]
    parents=[-1]*n
    q=deque([root])
    while q:
        poped=q.popleft()
        for neib in gl[poped]:
            if parents[neib]!=-1 or neib==root:continue
            parents[neib]=poped
            order.append(neib)
            q.append(neib)
    return order,parents

n,mod=map(int, input().split())
gl=[[] for _ in range(n)]
for _ in range(n-1):
    a,b=map(int, input().split())
    gl[a-1].append(b-1)
    gl[b-1].append(a-1)
order,pare=make_order_parents(n,gl)

dp=[1]*n
for v in order[::-1]:
    if pare[v]==-1:continue
    dp[pare[v]]*=(dp[v]+1)
    dp[pare[v]]%=mod

## pass the parent information to children. (children access thr parent info by this list.)
pare_val=[1]*n
ansl=[-1]*n
q=deque([0])
while q:
    poped=q.popleft()
    child_vals=[]
    for neib in gl[poped]:
        if ansl[neib]!=-1:continue
        child_vals.append(dp[neib])
    ls,rs=[1],[1] # left/right cumsums
    for cv in child_vals: ls.append((ls[-1]*(cv+1)%mod))
    for cv in child_vals[::-1]: rs.append((rs[-1]*(cv+1)%mod))

    ## calc ans from "all children" and "parent"
    child_num=len(gl[poped])-1
    if pare[poped]==-1:child_num+=1
    ans=ls[child_num]*pare_val[poped]
    ansl[poped]=ans%mod

    cind=0
    for neib in gl[poped]:
        if ansl[neib]!=-1:continue
        val=ls[cind]*rs[child_num-cind-1]*pare_val[poped]+1
        pare_val[neib]=val%mod # pass info to each child
        q.append(neib)
        cind+=1 # do not forget!

for a in ansl:print(a)