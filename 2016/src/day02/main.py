class Button(object):
    def __init__(self, value):
        self.value = value
        self._up = self
        self._down = self
        self._left = self
        self._right = self

    def clear(self):
        self._up = self
        self._down = self
        self._left = self
        self._right = self

    @property
    def up(self):
        return self._up

    @up.setter
    def up(self, val):
        self._up = val
        val._down = self

    @property
    def down(self):
        return self._down

    @down.setter
    def down(self, val):
        self._down = val
        val._up = self

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

ONE = Button("1")
TWO = Button("2")
THREE = Button("3")
FOUR = Button("4")
FIVE = Button("5")
SIX = Button("6")
SEVEN = Button("7")
EIGHT = Button("8")
NINE = Button("9")
A = Button("A")
BEE = Button("B")
CEE = Button("C")
DEE = Button("D")

ALL_BUTTONS = [ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, A, BEE, CEE, DEE]

COMMANDS = {
    'U': lambda x: x.up,
    'D': lambda x: x.down,
    'L': lambda x: x.left,
    'R': lambda x: x.right
}


def clear():
    [button.clear() for button in ALL_BUTTONS]


def get_init_config_1():
    clear()
    ONE.right = TWO
    ONE.down = FOUR
    TWO.right = THREE
    TWO.down = FIVE
    THREE.down = SIX
    FOUR.right = FIVE
    FOUR.down = SEVEN
    FIVE.right = SIX
    FIVE.down = EIGHT
    SIX.down = NINE
    SEVEN.right = EIGHT
    EIGHT.right = NINE
    return FIVE


def get_init_config_2():
    clear()
    ONE.down = THREE
    TWO.right = THREE
    TWO.down = SIX
    THREE.right = FOUR
    THREE.down = SEVEN
    FOUR.down = EIGHT
    FIVE.right = SIX
    SIX.right = SEVEN
    SIX.down = A
    SEVEN.right = EIGHT
    SEVEN.down = BEE
    EIGHT.right = NINE
    EIGHT.down = CEE
    A.right = BEE
    BEE.right = CEE
    BEE.down = DEE
    return FIVE


def get_commands():
    with open('../../resources/day2.txt', 'r') as fp:
        for line in fp:
            yield [COMMANDS[cmd] for cmd in line if cmd != '\n']


def run_with_config(config):
    current = config
    command_lists = get_commands()
    val = ''
    for commands in command_lists:
        for command in commands:
            current = command(current)
        val += current.value
    return val


def run_1():
    return run_with_config(get_init_config_1())


def run_2():
    return run_with_config(get_init_config_2())


if __name__ == '__main__':
    print run_1()
    print run_2()
