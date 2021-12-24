#include "xx_def.hpp"

struct Edge{
    int from,to,dist;
    Edge(int from, int to, int dist):from(from),to(to),dist(dist){}
    Edge(int from, int to):from(from),to(to),dist(1){}
    // add any properties
};

struct Node{
    ll value;
    Node(int value):value(value){}
    Node():value(0){}
    // add any properties
};

class Tree{
private:
    int n_; // # of nodes
    int root_ = 0; // default root is 0
    vector<vector<Edge>> edges_;
    vector<Node> nodes_;
    vector<Node> dfs_res_;
    vector<ll> dists_;
    
public:
    Tree(int n): n_(n), edges_(n), dists_(n,-1), nodes_(n){}
    void set_root(int root){ root_ = root; }
    void add_edge(Edge edge){ edges_[edge.from].push_back(edge); }
    void add_edge(int from, int to){ edges_[from].push_back(Edge(from,to,1)); } // add 1-cost edge
    void read_edges(){ int u,v; cin>>u>>v; u-=1; v-=1; add_edge(u,v); add_edge(v,u); } // read [undirected & 1-cost] edges
    void make_dists(){
        for(int i = 0 ; i < n_ ; i++) dists_[i] = -1;
        deque<int> q;
        dists_[root_] = 0;
        q.push_back(root_);
        while(!q.empty()){
            int poped = q.front(); q.pop_front();
            for(Edge e : edges_[poped]){
                if(dists_[e.to] != -1) continue;
                dists_[e.to] = dists_[poped] + e.dist;
                q.push_back(e.to);
            }
        }
    }
    void start_dfs(){
        if(dfs_res_.size()==0){
            for(int i = 0 ; i < n_ ; i++) dfs_res_.push_back(Node());
        }
        dfs(root_, -1);
    }
    Node dfs(int node, int parent){
        vector<pair<Node,Edge>> children;
        for(Edge e : edges_[node]){
            if(e.to == parent) continue;
            children.push_back(pair<Node,Edge>(nodes_[e.to],e));
        }
    }
};

int main(){
    // Tree t(4);
    return 0;
}