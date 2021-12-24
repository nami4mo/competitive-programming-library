# http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_2_A&lang=ja

INF = 10**9
n,e = map(int, input().split())
gl = [ [INF]*(n) for _ in range(n)]

for _ in range(e):
    s,t,d = map(int, input().split())
    gl[s][t] = d

dp = [ [INF]*(n) for _ in range(2**n) ]
dp[1][0] = 0
for i in range(2**n):
    for j in range(n):
        if dp[i][j] == INF: continue
        for k in range(n):
            if (i>>k)%2 == 1: continue
            dp[i|(1<<k)][k] = min(dp[i][j]+gl[j][k], dp[i|(1<<k)][k])

ans = INF
for i in range(1,n): 
    if gl[i][0] != INF:
        ans = min(ans, dp[2**n-1][i] + gl[i][0])

if ans == INF: ans = -1
print(ans)