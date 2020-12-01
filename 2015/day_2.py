import itertools
import os
from operator import mul
from functools import reduce


def get_input(in_file):
    open_file = open(in_file, 'r')
    for line in open_file:
        yield [int(item) for item in line.split('x')]
    open_file.close()


def size_needed(size):
    size_pairs = itertools.combinations(size, 2)
    size_areas = [size[0]*size[1] for size in size_pairs]
    return sum(2*size_area for size_area in size_areas) + min(size_areas)


def total_size_needed(sizes):
    return sum(size_needed(size) for size in sizes)


def length_for_bow(size):
    return reduce(mul, size)


def length_for_ribbon(size):
    small = min(size)
    size.remove(small)
    return 2*(small+min(size))


def total_ribbon_length(sizes):
    return sum(length_for_bow(size) + length_for_ribbon(size) for size in sizes)


if __name__ == '__main__':
    box_sizes = get_input(os.path.dirname(__file__) + '/input/day_2.input')
    print(total_size_needed(box_sizes))
    box_sizes = get_input(os.path.dirname(__file__) + '/input/day_2.input')
    print(total_ribbon_length(box_sizes))
