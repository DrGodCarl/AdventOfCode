
def line_to_length(line):
    lengths = line.split()
    lengths = [int(length) for length in lengths]
    return sorted(lengths)


def line_block_to_lengths(block):
    block = [row.split() for row in block]
    block = [[int(length) for length in row] for row in block]
    block = zip(*block[::-1])
    return [sorted(row) for row in block]


def get_lengths_row():
    with open('../../resources/day3.txt', 'r') as fp:
        for line in fp:
            yield line_to_length(line)


def get_lengths_column():
    with open('../../resources/day3.txt', 'r') as fp:
        lines = []
        should_buffer = 0
        for line in fp:
            lines.append(line)
            should_buffer = (should_buffer + 1) % 3
            if should_buffer:
                continue
            for triangle in line_block_to_lengths(lines):
                yield triangle
            lines = []


def is_valid(triangle):
    return triangle[0] + triangle[1] > triangle[2]


def run_1():
    return len([triangle for triangle in get_lengths_row() if is_valid(triangle)])


def run_2():
    return len([triangle for triangle in get_lengths_column() if is_valid(triangle)])

if __name__ == '__main__':
    print run_1()
    print run_2()
