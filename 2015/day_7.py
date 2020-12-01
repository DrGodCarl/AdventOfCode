import os

UNIVERSAL_PORT_MAP = {}

OPERATOR_MAP = {
    'AND': lambda a, b: a & b,
    'OR': lambda a, b: a | b,
    'LSHIFT': lambda a, b: a << b,
    'RSHIFT': lambda a, b: a >> b,
    'NOT': lambda a: ~a
}


def step_from_line(line):
    left, right = [item.strip() for item in line.split('->')]
    left = left.split()
    if len(left) == 3:
        left[0], left[1] = left[1], left[0]
    if len(left) > 1:
        left[0] = OPERATOR_MAP[left[0]]
    UNIVERSAL_PORT_MAP[right] = left


def populate_port_map(in_file):
    open_file = open(in_file, 'r')
    [step_from_line(line) for line in open_file]
    open_file.close()


def find_value_for_port(port):
    try:
        return int(port)
    except (TypeError, ValueError):
        pass
    try:
        return int(UNIVERSAL_PORT_MAP[port])
    except (TypeError, ValueError):
        pass
    values = UNIVERSAL_PORT_MAP[port]
    if len(values) == 1:
        return find_value_for_port(values[0])
    params = [find_value_for_port(value) for value in values[1:]]
    result = values[0](*params)
    UNIVERSAL_PORT_MAP[port] = result
    return result


if __name__ == '__main__':
    populate_port_map(os.path.dirname(__file__) + '/input/day_7.input')
    print(find_value_for_port('a'))
