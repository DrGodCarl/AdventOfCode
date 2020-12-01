import os


def get_input(in_file):
    open_file = open(in_file, 'r')
    for line in open_file:
        yield line.strip()
    open_file.close()


def number_of_mem_chars(line):
    return len(eval(line))


def diff_line(line):
    line_len = len(line)
    mem_len = number_of_mem_chars(line)
    return line_len - mem_len


def diff_lines(all_lines):
    return sum(diff_line(line) for line in all_lines)


def number_of_new_chars(line):
    return line.count('\\') + 2*line.count('"')


def reconstitute_diff_line(line):
    new_line = line.replace('\\', '\\\\').replace('"', '\\"')
    new_line = '"' + new_line + '"'
    assert line == eval(new_line)
    return len(new_line) - len(line)


def reconstitute_diff_lines(all_lines):
    return sum(reconstitute_diff_line(line) for line in all_lines)


if __name__ == '__main__':
    lines = get_input(os.path.dirname(__file__) + '/input/day_8.input')
    print(diff_lines(lines))
    lines = get_input(os.path.dirname(__file__) + '/input/day_8.input')
    print(reconstitute_diff_lines(lines))