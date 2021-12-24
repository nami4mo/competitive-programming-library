# http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A&lang=jp
from atcoder.maxflow import MFGraph

V,E = map(int, input().split())
g = MFGraph(V)
for _ in range(E):
    u,v,c = map(int, input().split())
    g.add_edge(u,v,c)

ans = g.flow(0,V-1)
print(ans)