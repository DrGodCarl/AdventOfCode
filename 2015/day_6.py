"""
This is a very slow solution but it does not blow out memory. So that's good.

Perhaps one day I'll parallelize this.
"""
import os
import re

FUNC_MAP_1 = {
    'toggle': lambda x: 0 if x else 1,
    'turn off': lambda x: 0,
    'turn on': lambda x: 1
}

FUNC_MAP_2 = {
    'toggle': lambda x: x + 2,
    'turn off': lambda x: x - 1 if x else 0,
    'turn on': lambda x: x + 1
}

NUMBER_PAIR_REGEX = re.compile(r'.*?(\d+,\d+).*?')


class Point(object):
    def __init__(self, x, y):
        self.x = x
        self.y = y


class Rectangle(object):
    def __init__(self, bottom_left, top_right):
        assert bottom_left.x <= top_right.x and bottom_left.y <= top_right.y
        self.bottom_left = bottom_left
        self.top_right = top_right

    def contains(self, item):
        return (self.bottom_left.x <= item.x <= self.top_right.x and
                self.bottom_left.y <= item.y <= self.top_right.y)


class CommandStep(object):
    def __init__(self, function, rectangle):
        self.function = function
        self.rectangle = rectangle


def extract_func(line, func_map):
    for key in func_map:
        if line.startswith(key):
            return func_map[key]


def extract_rect(line):
    def point_from_string(string):
        x, y = [int(i) for i in string.split(',')]
        return Point(x, y)

    regex_result = NUMBER_PAIR_REGEX.findall(line)
    bottom_point = point_from_string(regex_result[0])
    top_point = point_from_string(regex_result[1])
    return Rectangle(bottom_point, top_point)


def generate_command_step(line, func_map):
    return CommandStep(extract_func(line, func_map), extract_rect(line))


def extract_command_steps(in_file, func_map):
    open_file = open(in_file, 'r')
    result = [generate_command_step(line, func_map) for line in open_file]
    open_file.close()
    return result


def count_lights_over_range(commands, range_x, range_y):
    count = 0
    for i in range_x:
        print('Starting row ' + str(i))
        for j in range_y:
            point = Point(i, j)
            value = 0
            for command in commands:
                if command.rectangle.contains(point):
                    value = command.function(value)
            count += value
    return count

if __name__ == '__main__':
    command_steps = extract_command_steps(os.path.dirname(__file__) + '/input/day_6.input', FUNC_MAP_1)
    print(count_lights_over_range(command_steps, range(1000), range(1000)))
    command_steps = extract_command_steps(os.path.dirname(__file__) + '/input/day_6.input', FUNC_MAP_2)
    print(count_lights_over_range(command_steps, range(1000), range(1000)))