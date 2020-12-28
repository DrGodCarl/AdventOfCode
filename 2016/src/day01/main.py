class Direction(object):
    def __init__(self, axis, sign):
        self.axis = axis
        self.sign = sign
        self._left = None
        self._right = None

    @property
    def left(self):
        return self._left

    @left.setter
    def left(self, val):
        self._left = val
        val._right = self

    @property
    def right(self):
        return self._right

    @right.setter
    def right(self, val):
        self._right = val
        val._left = self


class Command(object):
    def __init__(self, text):
        self.dir_change = text[0]
        self.count = int(text[1:])

    def next_direction(self, cur_dir):
        return cur_dir.left if self.dir_change == 'L' else cur_dir.right


Y_AXIS = 'y'
X_AXIS = 'x'

NORTH = Direction(Y_AXIS, 1)
EAST = Direction(X_AXIS, 1)
SOUTH = Direction(Y_AXIS, -1)
WEST = Direction(X_AXIS, -1)

NORTH.left = WEST
WEST.left = SOUTH
SOUTH.left = EAST
EAST.left = NORTH


def get_commands():
    with open('../../resources/day1.txt', 'r') as fp:
        for text in fp.read().split(', '):
            yield Command(text)


def count_blocks(point):
    return abs(point[0]) + abs(point[1])


def run_1():
    commands = get_commands()
    cur_dir = NORTH
    grid = {Y_AXIS: 0, X_AXIS: 0}
    for cmd in commands:
        cur_dir = cmd.next_direction(cur_dir)
        grid[cur_dir.axis] += cmd.count * cur_dir.sign
    return count_blocks((grid[X_AXIS], grid[Y_AXIS]))


def run_2():
    commands = get_commands()
    cur_dir = NORTH
    grid = {Y_AXIS: 0, X_AXIS: 0}
    visited_points = set()
    for cmd in commands:
        cur_dir = cmd.next_direction(cur_dir)
        for step in range(cmd.count):
            cur_point = (grid[X_AXIS], grid[Y_AXIS])
            if cur_point in visited_points:
                return count_blocks(cur_point)
            visited_points.add(cur_point)
            grid[cur_dir.axis] += cur_dir.sign


if __name__ == '__main__':
    print run_1()
    print run_2()
