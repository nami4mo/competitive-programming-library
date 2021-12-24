from random import randint, shuffle, sample, choice
from copy import deepcopy

# TODO: make "Tree test case" generator

# functions for making test cases
def randi(a,b): return randint(a,b)
def randil(a,b,n): return [ randint(a,b) for _ in range(n) ]  # list of N elements (a <= n <= b)
def randil_2d(a,b,h,w): return [ [randint(a,b) for _i in range(w)] for _j in range(h)]  # 2-dim list of HxW elements (a <= n <= b)
def randil_unique(a,b,n): return sample(list(range(a,b+1)), n)  # N-unique elements (a <= n <= b)

def rands(char_list, n): return ''.join([choice(char_list) for _ in range(n)]) # (e.g.) rands(['B','W'], 4) -> 'BBWB'
def randcl(char_list, n): return [choice(char_list) for _ in range(n)] # (e.g.) randsl(['B','W'], 4) -> ['B', 'B', 'W', 'B']
def rands_2d(char_list, h, w): return [''.join([choice(char_list) for _ in range(w)]) for _ in range(h)] # (e.g.) rands_2dim(['#','.'], 3,2) -> ['#.', '..', '.#']
def randcl_2d(char_list, h, w): return [[choice(char_list) for _ in range(w)] for _ in range(h)] # (e.g.) randsl_2dim(['#','.'], 3,2) -> [['.', '#'], ['#', '#'], ['.', '#']]

def rand_unique_points(n, x1, x2, y1, y2): return sample([(x,y) for x in range(x1,x2+1) for y in range(y1,y2+1)], n)  # N-unique XY points ( x1 <= x <= x2, y1 <= y <= y2 ) 



def solve_main():
    ans = 0
    return ans


def solve_comp():
    ans = 0
    return ans


if __name__ == "__main__":
    LOOP_CNT = 100
    wa_cnt = 0
    for _ in range(LOOP_CNT):
        # generate test cases here
        ans_main = solve_main()
        ans_comp = solve_comp()
        if ans_main != ans_comp:
            print('--------')
            # print test case values here
            print(f'main: {ans_main}')
            print(f'comp: {ans_comp}')
            wa_cnt += 1
    print('=======')
    print(f'WA count: {wa_cnt}/{LOOP_CNT}')