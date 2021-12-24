#include "xx_def.hpp"

/* use Pair as key */
#include <boost/functional/hash.hpp>
unordered_map<P, vector<ll>, boost::hash<P>> mp;
unordered_set<P, boost::hash<P>> st;

const string alps = "abcdefghijklmnopqrstuvwxyz";
const string ALPs = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

void sample_char_map(){
    map<char, vector<int>> alp_d;
    for(int i = 0 ; i < 26; i++){
        vector<int> vec;
        alp_d[i+'a'] = vec;
    }
    for( char s : alps){
        alp_d[s].push_back(s-'a');
    }
}

void binary_search(){
    ll ok = 0;
    ll ng = 1e9+1;
    while(abs(ok-ng)>1){
        ll mid = (ok+ng)/2;
        bool ok_flag = true;
        //...
        if(ok_flag){
            ok = mid;
        } else {
            ng = mid;
        }
    }
    cout << ok << endl;
}


void cumsum(){
    int n; cin >> n;
    vector<ll> al(n); REP(i,n) cin >> al[i];
    vector<ll> cumsum(n+1,0);
    REP(i,n){
        cumsum[i+1] = cumsum[i] + al[i];
    }
    DEBUG(cumsum);
}

void cumsum2(){
    int n,m; cin >> n >> m;
    vector<vector<ll>> al(n, vector<ll>(m));
    REP(i,n){REP(j,m){ cin >> al[i][j]; }}
    
    vector<vector<ll>> cumsum(n+1, vector<ll>(m+1));
    REP(i,n){
        REP(j,m){
            cumsum[i+1][j+1] = cumsum[i+1][j] + cumsum[i][j+1] - cumsum[i][j] + al[i][j];
        }
    }
    DEBUG(cumsum);
}


int main()
{
    // sample_char_map();
    // binary_search();
    // cumsum();
    cumsum2();
}