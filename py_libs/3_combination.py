'''
    [K-bit full search]
'''

## use itertools.product  NOTE: これで十分 
## bit,n = 3,5 -> [(0,0,0,0,0), (0,0,0,0,1), (0,0,0,0,2), (0,0,0,1,0), ...., (2,2,2,2,2)]
from itertools import product
n = 5
bit = 3
ite = list(product(range(bit),repeat=n))
for pattern in ite:
    for i, v in enumerate(pattern):
        pass

## bit full search
from itertools import product
ite = list(product(range(2),repeat=n))
for pattern in ite:
    for i, v in enumerate(pattern):
        pass


## bit full search (normal)
n = 10
for i in range(2**n):
    comb = []
    for j in range(n):
        if ((i >> j) & 1): comb.append(j)


'''
    [順列列挙]
'''
## [(1,2,3),(1,2,4),(1,2,5),(1,3,2),...,(5,4,3)]
from itertools import permutations
n, k = 5, 3
ll = list(range(1,n+1))  # list of elements
perml = list(permutations(ll, k))


'''
    [組み合わせ列挙]
'''
# [(1,2,3),(1,2,4),(1,2,5),... (2,4,5),(3,4,5)]
from itertools import combinations
n, k = 5, 3
ll = list(range(1,n+1))  # list of elements
combl = list(combinations(ll, k))
print(combl)


'''
    [Combination]
'''
## N < 10**6
class Combination:
    def __init__(self, n_max=10**6, mod=10**9+7):
        # self._n_max = n_max
        self._fac, self._finv, self._inv = [0]*n_max, [0]*n_max, [0]*n_max
        self._fac[0], self._fac[1] = 1, 1
        self._finv[0], self._finv[1] = 1, 1
        self._inv[1] = 1
        self._mod = mod
        for i in range(2, n_max):
            self._fac[i] = self._fac[i - 1] * i % self._mod
            self._inv[i] = self._mod - self._inv[self._mod%i] * (self._mod // i) % self._mod
            self._finv[i] = self._finv[i - 1] * self._inv[i] % self._mod
    def com(self, n, r):
        if n < r: return 0
        if n < 0 or r < 0: return 0
        return self._fac[n] * (self._finv[r] * self._finv[n - r] % self._mod) % self._mod

    def perm(self,n,r):
        if n < r: return 0
        if n < 0 or r < 0: return 0
        return self._fac[n] * (self._finv[n-r] % self._mod) % self._mod

    def lucas(self, n, r): # nCr (mod self._mod(prime)) 
        if n < r: return 0 
        res = 1
        while n > 0:
            nq, rq = n//self._mod, r//self._mod
            nr, rr = n-nq*self._mod, r-rq*self._mod
            res *= self.com(nr, rr)
            res %= self._mod
            n, r = nq, rq
        return res

MOD = 10**9+7
comb = Combination(10**6, MOD)
comb.com(10,3)



## k < 10**7, n< 10**9
K_MAX = 10**7
MOD = 10**9 + 7
fac, finv, inv = [0]*(K_MAX+1), [0]*(K_MAX+1), [0]*(K_MAX+1)
def com_init():
    finv[0], finv[1] = 1, 1
    inv[1] = 1
    for i in range(2,K_MAX+1):
        inv[i] = MOD - inv[MOD%i] * (MOD // i) % MOD
        finv[i] = finv[i - 1] * inv[i] % MOD
def com(n, k):
    ans = 1
    for i in range(n-k+1,n+1):
        ans *= i
        ans %= MOD
    return (ans * finv[k]) % MOD


'''
    [modinv]
'''
def modinv(a,m):
    b, u, v = m, 1, 0
    while b:
        t = a//b
        a -= t*b
        a,b = b,a
        u -= t * v
        u,v = v,u
    u %= m
    return u

# (e.g.) 13/5 (mod MOD)
a = modinv(5,MOD)
ans = (13*a)%MOD


'''
    合同方程式
    find minimum x(>=0)
    such that [a*x = b (mod m)]
'''
# https://drken1215.hatenablog.com/entry/2020/12/20/015100
from math import gcd
def calc(a,b,m):
    gcd_am=gcd(a,m)
    if gcd_am!=1:
        if b%gcd_am!=0: return -1 # no answer
        a,b,m = a//gcd_am,b//gcd_am,m//gcd_am
    x=(b*modinv(a,m))%m
    return x