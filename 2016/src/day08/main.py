def print_screen(screen):
    return '\n'.join([''.join(row) for row in screen])


def rect_command(x, y, screen):
    for i in range(x):
        for j in range(y):
            screen[j][i] = '#'


def create_rect_command(text):
    vals = [int(i) for i in text.strip('rect ').split('x')]
    return lambda screen: rect_command(vals[0], vals[1], screen)


def rotate_row_command(y, n, screen):
    screen[y] = screen[y][-n:] + screen[y][:-n]


def create_row_rotate_command(text):
    vals = [int(i) for i in text.strip('rotate row y=').split(' by ')]
    return lambda screen: rotate_row_command(vals[0], vals[1], screen)


def rotate_column_command(x, n, screen):
    for _ in range(n):
        curr = screen[-1][x]
        for row in screen:
            row[x], curr = curr, row[x]


def create_column_rotate_command(text):
    vals = [int(i) for i in text.strip('rotate column x=').split(' by ')]
    return lambda screen: rotate_column_command(vals[0], vals[1], screen)


def create_command(text):
    if 'rect' in text:
        return create_rect_command(text)
    if 'row' in text:
        return create_row_rotate_command(text)
    if 'column' in text:
        return create_column_rotate_command(text)


def get_commands():
    with open('../../resources/day8.txt', 'r') as fp:
        for row in fp:
            yield create_command(row.strip())


def count_lights(screen):
    return sum([reduce(lambda i, p: i + 1 if p == '#' else i, row, 0) for row in screen])


def make_screen(x, y):
    screen = []
    for _ in range(y):
        screen.append(['.'] * x)
    return screen


def run_1():
    screen = make_screen(50, 6)
    for command in get_commands():
        command(screen)
    return count_lights(screen)


def run_2():
    screen = make_screen(50, 6)
    for command in get_commands():
        command(screen)
    return print_screen(screen)

if __name__ == '__main__':
    print run_1()
    print run_2()
