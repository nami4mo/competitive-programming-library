# http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_C&lang=jp

N,W = map(int, input().split())
vwl = [tuple(map(int, input().split())) for _ in range(N)]
dp = [0]*(W+1)
for wi in range(0,W+1):
    for v,w in vwl:
        if wi+w <= W:
            dp[wi+w] = max(dp[wi+w], dp[wi]+v)
print(max(dp))