'''
https://atcoder.jp/contests/arc100/tasks/arc100_c
https://ikatakos.com/pot/programming_algorithm/dynamic_programming/subset_convolution
'''

n=int(input())
al=list(map(int, input().split()))
'''
    init dp by al 
    dp=al[:]
'''
dp=[(a,-1) for a in al]

for j in range(n):
    bit=1<<j
    for i in range(2**n):
        if i&bit:
            '''
            dp[i] += dp[i^bit]
            merge dp[i^bit] into dp[i]

            ## pattern 2
            # dp[i] += dp[i | bit]
            '''
            a,b=dp[i]
            c,d=dp[i^bit]
            vs=[a,b,c,d]
            vs.sort(reverse=True)
            dp[i]=(vs[0],vs[1])

cmax=0
for a,b in dp[1:]:
    v=a+b
    cmax=max(cmax,v)
    print(cmax)