#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
typedef pair<ll,ll> P;
void NEWLINE(){cerr<<'\n';}
void COMMA(){cerr<<", ";}
void EPUTS(string x){cerr<<x<<'\n';}
#define DEBUG1(x) dbg(#x,x); NEWLINE();
#define DEBUG2(x1,x2) dbg(#x1,x1); COMMA(); dbg(#x2,x2); NEWLINE();
#define DEBUG3(x1,x2,x3) dbg(#x1,x1); COMMA(); dbg(#x2,x2); COMMA(); dbg(#x3,x3); NEWLINE();
#define DEBUG4(x1,x2,x3,x4) dbg(#x1,x1); COMMA(); dbg(#x2,x2); COMMA(); dbg(#x3,x3); COMMA(); dbg(#x4,x4); NEWLINE();
#define DEBUG_OVERLOAD(x1, x2, x3, x4, x5, ...) x5
#define DEBUG(...) DEBUG_OVERLOAD(__VA_ARGS__, DEBUG4, DEBUG3, DEBUG2, DEBUG1)(__VA_ARGS__)

template<class T> void dbg(string name, T x){cerr<<name<<": "<<x<<"";}
template<> void dbg<P>(string name, P x){cerr<<name<<": ("<<x.first<<","<<x.second<<")";}
template<class T> void dbg(string name, vector<T> xl){cerr<<name<<": "; for(T x: xl) cerr<<x<<" "; cerr<<"";}
template<> void dbg<P>(string name, vector<P> xl){cerr<<name<<": "; for(P x:xl){cerr<<"("<<x.first<<","<<x.second<<"), ";}cerr<<"";}
template<class T> void dbg(string name, vector<vector<T>> xl){ cerr<<name<<": \n"; int ml=1;for(vector<T> row: xl){for(T x:row){stringstream sstm; sstm<<x; ml=max(ml,(int)sstm.str().size());}}; for(vector<T> row: xl){{for(T x:row) cerr<<std::right<<std::setw(ml+1)<<x;} cerr << '\n';}}
template<class T> void dbg(string name, set<T> xl){cerr<<name<<": "; for(T x:xl)cerr<<x<<" ";cerr<<'\n';}
template<class T> void dbg(string name, multiset<T> xl){cerr<<name<<": "; for(T x:xl)cerr<<x<<" ";cerr<<'\n';}
template<class T> void dbg(string name, unordered_set<T> xl){cerr<<name<<": "; for(T x:xl)cerr<<x<<" ";cerr<<'\n';}
template<class T> void dbg(string name, unordered_multiset<T> xl){cerr<<name<<": "; for(T x:xl)cerr<<x;cerr<<'\n';}
template<class T, class U> void dbg(string name, map<T,U> xl){cerr<<name<<": \n"; for(auto &[k,v]:xl)cerr<<"  "<<k<<": "<<v<<'\n';}
template<class T, class U> void dbg(string name, unordered_map<T,U> xl){cerr<<name<<": \n"; for(auto &[k,v]:xl)cerr<<"  "<<k<<": "<<v<<'\n';}
