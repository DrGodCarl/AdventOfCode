import os


def get_row(line):
    return [1 if char == '#' else 0 for char in line.strip()]


def get_input(in_file):
    with open(in_file, 'r') as open_file:
        return [get_row(line) for line in open_file]


def count_of_on_neighbors(current_state, x, y):
    return sum(current_state[this_x][this_y] for
               this_x in range(x-1, x+2) for
               this_y in range(y-1, y+2)
               if 0 <= this_x < len(current_state) and 0 <= this_y < len(current_state[this_x]))\
        - current_state[x][y]


def new_state(current_state, x, y, corners_on):
    if corners_on:
        max_x = len(current_state) - 1
        max_y = len(current_state[x]) - 1
        if (x, y) in [(0, 0), (0, max_y), (max_x, 0), (max_x, max_y)]:
            return 1
    current = current_state[x][y]
    count = count_of_on_neighbors(current_state, x, y)
    if current == 1 and count in [2, 3]:
        return 1
    if current == 0 and count == 3:
        return 1
    return 0


def iterate_once(current_state, corners_on):
    return [[new_state(current_state, x, y, corners_on) for y in range(len(current_state[x]))]
            for x in range(len(current_state))]


def iterate_many(current_state, num_iter, corners_on=False):
    for _ in range(num_iter):
        current_state = iterate_once(current_state, corners_on)
    return current_state


def count_lights(count_state):
    return sum([light for row in count_state for light in row])


if __name__ == '__main__':
    light_array = get_input(os.path.dirname(__file__) + '/input/day_18.input')
    print(count_lights(iterate_many(light_array, 100)))
    light_array[0][0] = 1
    light_array[-1][0] = 1
    light_array[0][-1] = 1
    light_array[-1][-1] = 1
    print(count_lights(iterate_many(light_array, 100, corners_on=True)))
