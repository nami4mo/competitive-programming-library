#include "xx_def.hpp"

vector<ll> dijkstra(int start, int n, vector<vector<P>> &g){
    vector<ll> d(n,INF);
    d[start] = 0;
    priority_queue<P, vector<P>, greater<P>> q;
    q.push(P(0,start));

    while(!q.empty()){
        P dist_v = q.top();
        ll dist = dist_v.first;
        ll v = dist_v.second;
        q.pop();
        if( d[v] < dist ) continue;
        for( P v_cost : g[v] ){
            ll next_v = v_cost.first;
            ll cost = v_cost.second;
            if( d[next_v] > d[v] + cost ){
                d[next_v] = d[v] + cost;
                q.push(P(d[next_v], next_v));
            }
        }
    }
    return d;
}

int main(){
    
}