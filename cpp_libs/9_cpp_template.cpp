#include <bits/stdc++.h>
#if __has_include(<atcoder/all>)
#include <atcoder/all>
using namespace atcoder;
#endif
#if __has_include(<local_debug.hpp>)
#include <local_debug.hpp>
#else
#define DEBUG(...)
#endif

using namespace std;

using ll = long long;
using P = pair<ll, ll>;
template <class T>
using Vec = vector<T>;

#define FOR(i, a, b) for (ll i = a; i < b; i++)   // for i in range(a,b)
#define REP(i, n) for (ll i = 0; i < n; i++)      // for i in range(b)
#define FORD(i, a, b) for (ll i = a; i > b; i--)  // for i in range(a,b,-1)
#define ALL(x) x.begin(), x.end()
#define IN(y, h) !(y < 0 || h <= y)
#define OUT(y, h) (y < 0 || h <= y)

const Vec<P> DYXS = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

const int IINF = 1'001'001'001;
const ll INF = 1'001'001'001'001'001'001ll;
const ll MOD = 1'000'000'007;
// const ll MOD = 998244353;
// using mint = modint1000000007;
// using mint = modint998244353;

void solve() {
    //
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    // DEBUG("local");
    solve();
    return 0;
}