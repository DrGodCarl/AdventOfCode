import os

INCREMENT_MAP = {
    '>': [1, 0],
    '^': [0, 1],
    '<': [-1, 0],
    'v': [0, -1],
}


def get_input(in_file):
    open_file = open(in_file, 'r')
    out = open_file.read()
    open_file.close()
    return out


def next_house(instruction, current_location):
    increment = INCREMENT_MAP[instruction]
    return current_location[0] + increment[0], current_location[1] + increment[1]


def execute_instructions(instructions):
    current_location = (0, 0)
    house_set = set()
    house_set.add(current_location)
    for instruction in instructions:
        current_location = next_house(instruction, current_location)
        house_set.add(current_location)
    return house_set


if __name__ == '__main__':
    instructs = get_input(os.path.dirname(__file__) + '/input/day_3.input')
    print(len(execute_instructions(instructs)))
    robo = instructs[::2]
    santa = instructs[1::2]
    robo_houses = execute_instructions(robo)
    santa_houses = execute_instructions(santa)
    houses = robo_houses.union(santa_houses)
    print(len(houses))
