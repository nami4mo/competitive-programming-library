# 中央値を扱うためのクラス
# https://atcoder.jp/contests/abc127/tasks/abc127_f

'''
# e.g.
[1,4,5,7,9]
- get_med() -> [5] # list
- get_left_sum() -> 5, get_right_sum() -> 16

[1,4,5,7,9,10]
- get_med() -> [5,7] # ;ist
- get_left_sum() -> 5, get_right_sum() -> 19
'''

from heapq import heappop, heappush
class MedianContainer:
    def __init__(self):
        self._left_q = []
        self._right_q = []
        self._left_sum = 0
        self._right_sum = 0
        self._med1, self._med2 = None, None
        self._med = None

    def insert(self, x):
        if not any([self._med, self._med1, self._med2]): # First insert
            self._med = x
        elif self._med is None and self._med1 is not None and self._med2 is not None:
            if x <= self._med1:
                heappush(self._left_q, x*(-1))
                heappush(self._right_q, self._med2)
                self._left_sum += x
                self._right_sum += self._med2
                self._med = self._med1
            elif x >= self._med2:
                heappush(self._left_q, self._med1*(-1))
                heappush(self._right_q, x)
                self._left_sum += self._med1
                self._right_sum += x
                self._med = self._med2
            else:
                heappush(self._left_q, self._med1*(-1))
                heappush(self._right_q, self._med2)
                self._left_sum += self._med1
                self._right_sum += self._med2
                self._med = x
            self._med1, self._med2 = None, None
        else:
            if x <= self._med:
                heappush(self._left_q, x*(-1))
                self._med1 = heappop(self._left_q)*(-1)
                self._left_sum += x
                self._left_sum -= self._med1
                self._med2 = self._med
            else:
                heappush(self._right_q, x)
                self._right_sum += x
                self._med1 = self._med
                self._med2 = heappop(self._right_q)
                self._right_sum -= self._med2
            self._med = None

        
    def get_med(self):
        if self._med is not None:
            return [self._med]
        else:
            return [self._med1, self._med2]


    def get_left_sum(self):
        return self._left_sum


    def get_right_sum(self):
        return self._right_sum


    def get_half_len(self):
        return len(self._left_q)