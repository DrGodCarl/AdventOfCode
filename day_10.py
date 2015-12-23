import itertools

INPUT = '1321131112'


def look_and_say(string):
    group = itertools.groupby(string)
    return ''.join(str(len(list(count))) + num for num, count in group)


def iterate_look_and_say(string_in, iterations):
    for i in range(iterations):
        string_in = look_and_say(string_in)
    return string_in


if __name__ == '__main__':
    print(len(iterate_look_and_say(INPUT, 50)))
