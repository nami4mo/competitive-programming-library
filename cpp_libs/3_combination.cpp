#include "xx_def.hpp"

const int MOD = 1000000007;
const int MAX = 510000;
long long fac[MAX], finv[MAX], inv[MAX];
void com_init() {
    fac[0] = fac[1] = 1;
    finv[0] = finv[1] = 1;
    inv[1] = 1;
    for (int i = 2; i < MAX; i++){
        fac[i] = fac[i - 1] * i % MOD;
        inv[i] = MOD - inv[MOD%i] * (MOD / i) % MOD;
        finv[i] = finv[i - 1] * inv[i] % MOD;
    }
}

long long com(int n, int k){
    if (n < k) return 0;
    if (n < 0 || k < 0) return 0;
    return fac[n] * (finv[k] * finv[n - k] % MOD) % MOD;
}

// not validated
ll perm(int n, int r){
    if(n<r) return 1;
    if(n<0 || r<0) return 1;
    return fac[n] * (finv[n-r] % MOD) % MOD; 
}
