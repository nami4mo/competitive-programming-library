#include "xx_def.hpp"

const vector<int> al = {1, 2, 4, 4, 7, 9, 9, 10};
const set<int> st = {1, 2, 4, 7, 9, 10, 13, 15};
const multiset<int> mst = {1, 2, 4, 4, 7, 9, 9, 10, 13};

void binary_search_template(){
    int x;

    x = 3;
    if( binary_search(al.begin(), al.end(), x) ){ 
        cout << "x exists" << endl;
    }
    
    // x以上の最小要素のindex/val
    x = 4;
    auto iter1 = lower_bound(ALL(al), x);
    int ind1 = iter1 - al.begin();
    if( iter1 == al.end() ){
        // no such val
    }
    int val1 = *iter1;
    cout << ind1 << ": " << val1 << endl;

    
    // xより大きい最小要素のindex/val
    x = 4;
    auto iter2 = upper_bound(ALL(al), x);
    int ind2 = iter2 - al.begin();
    if( iter2 == al.end() ){
        // no such val
    }
    int val2 = *iter2;
    cout << ind2 << ": " << val2 << endl;

    
    //x以下の最大要素のindex/value
    x = 4;
    auto iter3 = upper_bound(ALL(al), x);
    int ind3 = iter3 - al.begin() - 1;
    if( ind3 < 0 ){
        // no such val
    }
    int val3 = al[ind3];
    cout << ind3 << ": " << val3 << endl;


    //x未満の最大要素のindex/val
    x = 4;
    auto iter4 = lower_bound(ALL(al), x);
    int ind4 = iter4 - al.begin() - 1;
    if( ind4 < 0 ){
        // no such val
    }
    int val4 = al[ind4];
    cout << ind4 << ": " << val4 << endl;

}


void set_binary_search_template(){
    int x;

    // x以上の最小要素のval
    x = 5;
    auto iter1 = st.lower_bound(x);
    if( iter1 == st.end() ){
        // no such val
    }
    int val1 = *iter1;
    cout << val1 << endl;
    // int cnt = distance(st.begin(), iter1);


    // xより大きい最小要素のval
    x = 9;
    auto iter2 = st.upper_bound(x);
    if( iter2 == st.end() ){
        // no such val
    }
    int val2 = *iter2;
    cout << val2 << endl;


    // x以下の最大要素のval
    x = 9;
    auto iter3 = st.upper_bound(x);
    if( iter3 == st.begin() ){
        // no such val
    }
    int val3 = *(--iter3);
    cout << val3 << endl;


    // x未満の最大要素のval
    x = 9;
    auto iter4 = st.lower_bound(x);
    if( iter4 == st.begin() ){
        // no such val
    }
    int val4 = *(--iter4);
    cout << val4 << endl;

}


int main(){
    binary_search_template();
    set_binary_search_template();
}

