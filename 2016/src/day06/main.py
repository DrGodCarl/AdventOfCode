from collections import defaultdict
from collections import Counter


def get_rows():
    with open('../../resources/day6.txt', 'r') as fp:
        for row in fp:
            yield row.strip()


def count_letters():
    position_to_count = defaultdict(Counter)
    for row in get_rows():
        for i in range(len(row)):
            position_to_count[i].update(row[i])
    return position_to_count


def create_words(letter_count_map, most_common):
    pos = 0
    word = ''
    while len(letter_count_map[pos]):
        word += letter_count_map[pos].most_common()[0 if most_common else -1][0]
        pos += 1
    return word


def create_words_most_common(letter_count_map):
    return create_words(letter_count_map, True)


def create_words_least_common(letter_count_map):
    return create_words(letter_count_map, False)


def run_1():
    return create_words_most_common(count_letters())


def run_2():
    return create_words_least_common(count_letters())

if __name__ == '__main__':
    print run_1()
    print run_2()
