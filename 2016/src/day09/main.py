from itertools import takewhile
from itertools import islice


def get_text():
    with open('../../resources/day9.txt', 'r') as fp:
        return fp.read().strip()


def decompress(data, recurse):
    answer = 0
    chars = iter(data)
    for c in chars:
        if c == '(':
            n, m = map(int, [''.join(takewhile(lambda c: c not in 'x)', chars)) for _ in range(2)])
            s = ''.join(islice(chars, n))
            answer += (decompress(s, recurse) if recurse else len(s))*m
        else:
            answer += 1
    return answer


def run_1():
    return decompress(get_text(), False)


def run_2():
    return decompress(get_text(), True)

if __name__ == '__main__':
    print run_1()
    print run_2()