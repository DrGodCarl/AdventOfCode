import os
import itertools

GOAL_VALUE = 150


def get_input(in_file):
    with open(in_file, 'r') as open_file:
        return [int(line) for line in open_file]


def find_combinations(containers, goal, min_only=False):
    total = 0
    for i in range(1, len(containers) + 1):
        combos = itertools.combinations(containers, i)
        for combo in combos:
            total += 1 if sum(combo) == goal else 0
        if total and min_only:
            return total
    return total

if __name__ == '__main__':
    container_sizes = get_input(os.path.dirname(__file__) + '/input/day_17.input')
    print(find_combinations(container_sizes, GOAL_VALUE))
    print(find_combinations(container_sizes, GOAL_VALUE, min_only=True))