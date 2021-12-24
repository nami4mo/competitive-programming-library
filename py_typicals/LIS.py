# http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_D
# https://qiita.com/python_walker/items/d1e2be789f6e7a0851e5

from bisect import bisect_left
INF = 10**10

n = int(input())
# al = list(map(int, input().split()))
al = [int(input()) for _ in range(n)]

dp = [INF]*(n+1)
dp[0] = -INF

for i, a in enumerate(al):
    ind = bisect_left(dp, a) # max index of "value <= a"
    dp[ind] = a
ans = bisect_left(dp, INF) - 1 # cnt of "value < INF" and remove 0-index
print(ans)