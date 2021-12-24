''' [input] '''
import sys
input = sys.stdin.readline
input().rstrip()  # remove last '\n'


''' [for DFS] '''
sys.setrecursionlimit(10**6)


''' [binary search] '''
# ng, ok = 0, 10**9+1
ok, ng = 0, 10**9+1
while abs(ok-ng) > 1:
    mid = (ok+ng)//2
    res = True
    if res:
        ok = mid
    else:
        ng = mid
print(ok)


''' [alphabet] '''
alps = 'abcdefghijklmnopqrstuvwxyz'  # string.ascii_lowercase
alps = [chr(ord('a')+i) for i in range(26)]
alp_d = {chr(ord('a') + i): 0 for i in range(26)}
ALPS = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'  # string.ascii_uppercase
ALPs = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'

ALPS = alps.upper()
alps = ALPS.lower()


''' [3d-list] '''
l, m, n = 5, 5, 5
dp = [[[0]*l for _ in range(m)] for _ in range(n)]


''' [2-dim cumulative sum] '''
h, w = 4, 3
al = [[1, 2, 6], [3, 10, 3], [3, 4, 1], [1, 3, 4]]
csums = [[0]*(w+1) for _ in range(h+1)]
for i in range(h):
    for j in range(w):
        csums[i+1][j+1] = csums[i+1][j] + \
            csums[i][j+1] - csums[i][j] + al[i][j]

# [ (y0,x0) ~ (y1,x1) ] の合計取得（両端含む）
y1, x1 = 2, 1
y0, x0 = 1, 1
val = csums[y1+1][x1+1] - csums[y1+1][x0] - csums[y0][x1+1] + csums[y0][x0]


''' [math.ceil] 繰り上がり '''
a, b = 10, 3
v = (a-1)//b + 1


''' [Run Length Encoding] '''
al = [1, 1, 5, 3, 3, 3, 3, 3, 4, 4, 1, 2, 2]
cntl = []
prev = al[0]
cnt = 1
for a in al[1:]:
    if prev == a:
        cnt += 1
    else:
        cntl.append((prev, cnt))
        cnt = 1
        prev = a
cntl.append((prev, cnt))


''' [線分の交差判定] '''


class P:
    def __init__(self, x, y):
        self.x = x
        self.y = y


def cross(a, b, c, d):  # cross? AB CD
    s = (a.x - b.x) * (c.y - a.y) - (a.y - b.y) * (c.x - a.x)
    t = (a.x - b.x) * (d.y - a.y) - (a.y - b.y) * (d.x - a.x)
    if s*t > 0:
        return False
    s = (c.x - d.x) * (a.y - c.y) - (c.y - d.y) * (a.x - c.x)
    t = (c.x - d.x) * (b.y - c.y) - (c.y - d.y) * (b.x - c.x)
    if s*t > 0:
        return False
    return True


''' bit 部分集合 3**n '''
# https://atcoder.jp/contests/dp/submissions/27756431
n = 3
for i in range(1 << n):
    # 片方が空集合になるのを認めない場合
    bits = i
    while True:
        bits = (bits-1) & i
        comp = bits ^ i
        if bits <= i//2:
            break  # (bits,comp), (comp,bits) の片方だけ見れたらOK（両方見ると重い）
        # print('processing here', bits,comp)

    # 片方が空集合でもOKな場合
    # bits = i
    # comp = bits ^ i
    # while True:
    #     # print('processing here222', bits, comp)
    #     bits = (bits-1) & i
    #     comp = bits ^ i
    #     if bits <= i//2: break


# for(ll i=(bits-1)&bits ; i>0 ; i=(i-1)&bits){
