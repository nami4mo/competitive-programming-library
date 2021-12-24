''' [木の直径] '''
import heapq
from collections import deque


def bfs(start, g, visited):
    q = deque([start])
    visited[start] = 0
    last_node = 0
    while q:
        curr_node = q.popleft()
        for next_node in g[curr_node]:
            if visited[next_node] >= 0:
                continue
            visited[next_node] = visited[curr_node] + 1
            q.append(next_node)
            last_node = next_node
    return last_node


n = int(input())
gl = [[] for _ in range(n)]
visited = [-1] * (n)
for i in range(n-1):
    a, b = map(int, input().split())
    a -= 1
    b -= 1
    gl[a].append(b)
    gl[b].append(a)

node_a = bfs(0, gl, visited)
visited = [-1] * (n)
node_b = bfs(node_a, gl, visited)
# print(visited[node_b])
print(node_a, node_b)


''' [01-BFS] '''
# https://atcoder.jp/contests/abc176/tasks/abc176_d
# abc176_d2.py


'''
    [dijkstra] 
    - 単一始点用（1 対 全）/負の辺があるときは使用不可
    - O(|E|*|logV|)... 全ノードが繋がっているとき(|E|=|V|**2)などは要注意
    - 拡張dijkstra... abc164_e.py
    - 最短経路復元は、コメント部分を外す
'''


def dijkstra(s, n, g):  # s: start, n: |V|, g; glaph (to,cost)
    INF = 10**18
    d = [INF] * n
    # -- record the prev vertex of each one for restoring the route --
    prev_vl = [-1]*n
    d[s] = 0
    que = []  # (a,b): (a: shortest dist, b: node)
    heapq.heappush(que, (0, s))

    while que:
        dist, v = heapq.heappop(que)
        if d[v] < dist:
            continue  # if v has been already used -> continue
        for next_v, cost in g[v]:
            if d[next_v] > d[v] + cost:
                d[next_v] = d[v] + cost
                prev_vl[next_v] = v
                heapq.heappush(que, (d[next_v], next_v))
    # resotre the route
    # goal = n-1 # set goal here (do you need loop?)
    # route = [goal]
    # curr_v = goal
    # while True:
    #     prev_v = prev_vl[curr_v]
    #     if prev_v == -1: break
    #     route.append(prev_v)
    #     curr_v = prev_v
    # route = route[::-1]
    # return d, route
    return d


n, m = map(int, input().split())
gl = [[] for _ in range(n)]  # (to, cost)
for _ in range(m):
    a, b, c = map(int, input().split())
    a -= 1
    b -= 1
    gl[a].append((b, c))
    gl[b].append((a, c))
start = 0
dist = dijkstra(start, n, gl)


''' [warshall_floyd] '''


def warshall_floyd(d, n):
    #d[i][j]: iからjへの最短距離
    for k in range(n):
        for i in range(n):
            for j in range(n):
                d[i][j] = min(d[i][j], d[i][k] + d[k][j])
    return d


'''
    [bellman_ford]
    # g...  list of edges [ (from,to,cost), (from,to,cost), ...]
    # 負の閉路がゴールにたどりつくか判定するには、n-1回のループの後、距離更新時にINFを伝搬させるループをさらにn-1回行う (abc061_d)
    # もしくは、ゴールから逆順に辿った頂点 と スタートから辿った頂点 の＆をとって、不要な頂点（ゴールにたどり着かない閉路）を消す
'''


def bellman_ford(s, n, g):  # s: start, n: |V|, g; glaph
    INF = 10**18
    d = [INF]*n
    d[s] = 0
    for i in range(n):  # max n-1 loops. if update d[] in n-th loop -> negative cycle
        update = False
        for v_from, v_to, cost in g:
            if d[v_from] != INF and d[v_to] > d[v_from] + cost:
                d[v_to] = d[v_from] + cost
                update = True
        if not update:
            return d
    else:  # if not break until n-th loop -> detect negative cycle
        # may do something for negatice cycle
        return None


'''
    make dfs-tour order
'''


def make_dfs_order(n, gl, root=0):
    res = []
    vis = [False]*n
    q = deque()
    q.append(root)
    vis[root] = True
    while q:
        poped = q.popleft()
        res.append(poped)
        for neib in gl[poped]:
            if vis[neib]:
                continue
            vis[neib] = True
            q.appendleft(neib)
    return res
