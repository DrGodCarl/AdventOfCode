import os
import re

SPECIFIC_EXCLUSIONS = [
    'ab',
    'cd',
    'pq',
    'xy',
]

ALL_VOWELS = ['a', 'e', 'i', 'o', 'u']

DOUBLE_LETTER_REGEX = re.compile(r'.*(.)\1.*', re.IGNORECASE)

PAIR_MATCHING_REGEX = re.compile(r'.*(..).*\1.*', re.IGNORECASE)

ONE_LETTER_BETWEEN_REGEX = re.compile(r'.*(.).\1.*', re.IGNORECASE)

def get_input(in_file):
    open_file = open(in_file, 'r')
    for line in open_file:
        yield line
    open_file.close()


def verify_letters(string, letters):
    return sum(string.count(letter) for letter in letters) > 2


def matches_regex(string, regex):
    return regex.match(string)


def check_specifics(string, specifics):
    return not any(specific in string for specific in specifics)


def reduce_list_1(words):
    return [word for word in words if
            check_specifics(word, SPECIFIC_EXCLUSIONS) and
            matches_regex(word, DOUBLE_LETTER_REGEX) and
            verify_letters(word, ALL_VOWELS)]


def reduce_list_2(words):
    return [word for word in words if
            matches_regex(word, PAIR_MATCHING_REGEX) and
            matches_regex(word, ONE_LETTER_BETWEEN_REGEX)]

if __name__ == '__main__':
    word_list = get_input(os.path.dirname(__file__) + '/input/day_5.input')
    print(len(reduce_list_1(word_list)))
    word_list = get_input(os.path.dirname(__file__) + '/input/day_5.input')
    print(len(reduce_list_2(word_list)))
