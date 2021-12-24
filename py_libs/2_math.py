'''　[LCM]　'''
from math import gcd
def lcm(x, y): return (x * y) // gcd(x, y)


''' [分数] '''


class Frac:
    def __init__(self, nume, deno):
        self.nume = nume
        self.deno = deno

    def __lt__(self, rh): return self.nume * rh.deno < self.deno * rh.nume
    def __gt__(self, rh): return self.nume * rh.deno > self.deno * rh.nume
    def __eq__(self, rh): return self.nume * rh.deno == self.deno * rh.nume
    def __le__(self, rh): return self.nume * rh.deno <= self.deno * rh.nume
    def __ge__(self, rh): return self.nume * rh.deno >= self.deno * rh.nume
    def __ne__(self, rh): return self.nume * rh.deno != self.deno * rh.nume
    def __repr__(self): return '{}/{}'.format(self.nume, self.deno)


'''　[約数列挙]　'''
n = 36
divs = []
for i in range(1, int(n**0.5)+1):
    if n % i == 0:
        divs.append(i)
        if i*i != n:
            divs.append(n//i)


'''　[素数列挙（エラトステネスの篩）] (N < 10^7)　'''


def primes(n):
    is_prime = [True] * (n + 1)
    is_prime[0] = False
    is_prime[1] = False
    for i in range(2, int(n**0.5) + 1):
        if not is_prime[i]:
            continue
        for j in range(i * 2, n + 1, i):
            is_prime[j] = False
    return [i for i in range(n + 1) if is_prime[i]]


''' [素因数分解（dict）] '''
# (e.g) 60 -> {2:2, 3:1, 5:1}   1 -> {}
# NOTE: p_factorization... O(√N),  GCD... O(logN)
# GCDを求めるだけなら、素因数分解するより math.gcd の方が圧倒的に高速


def p_factorization(n):
    if n == 1:
        return {}
    pf_cnt = {}
    temp = n
    for i in range(2, int(-(-n**0.5//1))+1):
        if temp % i == 0:
            cnt = 0
            while temp % i == 0:
                cnt += 1
                temp //= i
            pf_cnt[i] = cnt

    if temp != 1:
        pf_cnt[temp] = 1
    return pf_cnt


''' [素因数分解（tuple）] '''
# 60 -> [(2,2), (3,1), (5,1)]  (list of tuples)


def p_factorization_t(n):
    if n == 1:
        return []
    pf_cnt = []
    temp = n
    for i in range(2, int(-(-n**0.5//1))+1):
        if temp % i == 0:
            cnt = 0
            while temp % i == 0:
                cnt += 1
                temp //= i
            pf_cnt.append((i, cnt))

    if temp != 1:
        pf_cnt.append((temp, 1))
    return pf_cnt


''' [素因数分解（osa_k法）] '''
# [3,4,6] -> [ {3:1}, {2:2}, {2:1, 3:1} ]  (list of)
# O( NloglogN + |V|logN )... 前処理 NloglogN（エラトステネスの篩）, 各値に対して logN


def p_factorization_osa_k(vl):
    vmax = max(vl)
    min_primes = list(range(vmax+1))  # initialize all values by its own value
    for i in range(2, int(vmax**0.5) + 1):
        if min_primes[i] != i:
            continue  # if not prime
        for j in range(i, vmax+1, i):
            # -> if min_primes[j] == j: min_primes[j] = i
            min_primes[j] = min(min_primes[j], i)
    results = []
    for v in vl:
        p_cnt = {}
        curr_v = v
        while curr_v > 1:
            min_p = min_primes[curr_v]
            p_cnt.setdefault(min_p, 0)
            p_cnt[min_p] += 1
            curr_v //= min_p
        results.append(p_cnt)
    return results


''' [不定方程式] ax + by = 1 '''


def ext_gcd(a, b, x, y):
    if b == 0:
        x[0] = 1
        y[0] = 0
        return a
    d = ext_gcd(b, a % b, y, x)
    y[0] -= a//b * x[0]
    return d


a, b = 5, -3
if gcd(a, b) == 1:
    x, y = [0], [0]
    ext_gcd(8, 6, x, y)
    x, y = x[0], y[0]
    print(x, y)
else:
    pass  # no ans


'''
    [modinv]
'''


def modinv(a, m):
    b, u, v = m, 1, 0
    while b:
        t = a//b
        a -= t*b
        a, b = b, a
        u -= t * v
        u, v = v, u
    u %= m
    return u


'''
    find minimum x(>=0) 
    [a*x = b (mod m)]
'''


def calc(a, b, m):
    gcd_am = gcd(a, m)
    if gcd_am != 1:
        if b % gcd_am != 0:
            return -1  # no answer
        a, b, m = a//gcd_am, b//gcd_am, m//gcd_am
    x = (b*modinv(a, m)) % m
    return x


''' [繰り返し2乗法] '''
# NOTE: pow(x,n) か pow(x,n,MOD) で十分高速
MOD = 10**9+7


def pow_k(x, n):
    if n == 0:
        return 1
    K = 1
    while n > 1:
        if n % 2 != 0:
            K *= x
        x *= x
        n //= 2
        x %= MOD
    return K * x


''' 行列累乗 '''


def multi_mat(x, y, mod):
    row = len(x)
    mid = len(y)  # len(x[0])
    col = len(y[0])
    res = [[0]*col for _ in range(row)]
    for i in range(row):
        for j in range(col):
            for k in range(mid):
                res[i][j] += x[i][k]*y[k][j]
                res[i][j] %= mod
    return res


def pow_mat(x, n, mod):
    size = len(x)
    res = [[0]*size for _ in range(size)]
    for i in range(size):
        res[i][i] = 1
    if n == 0:
        return res
    xk = x
    while n > 1:
        if n % 2 != 0:
            res = multi_mat(res, xk, mod)
        xk = multi_mat(xk, xk, mod)
        n >>= 1
    return multi_mat(res, xk, mod)


'''
sum_of_floor (from ACL)
'''


def sum_of_floor(n, m, a, b):
    ans = 0
    if a >= m:
        ans += (n - 1) * n * (a // m) // 2
        a %= m
    if b >= m:
        ans += n * (b // m)
        b %= m
    y_max = (a * n + b) // m
    x_max = (y_max * m - b)
    if y_max == 0:
        return ans
    ans += (n - (x_max + a - 1) // a) * y_max
    ans += sum_of_floor(y_max, a, m, (a - x_max % a) % a)
    return ans
