
class Chip(object):
    def __init__(self, value):
        self.value = value


class Bot(object):
    def __init__(self, number, rule):
        self.lower = None
        self.higher = None
        self.number = number
        self.rule = rule

    def consume_chip(self, chip):
        if not self.lower:
            self.lower = chip
        else:
            self.lower, self.higher = sorted((self.lower, chip), key=lambda c: c.value)
            self.rule(self)


def get_commands():
    with open('../../resources/day10.txt', 'r') as fp:
        for row in fp:
            yield row.strip()


def run_1():
    pass

if __name__ == '__main__':
    print run_1()
