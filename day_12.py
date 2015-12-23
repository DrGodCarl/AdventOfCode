import os
import re
import json

# Well the regex that worked for the first question was pretty terrible for the second.
# Got to this point for identifying chunks with 'red' in them and then gave up:
# .*?({[^}{]*[^\[]*"red"[^\]]*[^}]*}).*?
NUMBER_REGEX = re.compile(r'.*?(\-?[1-9][0-9]*).*?')


def get_input(in_file):
    open_file = open(in_file, 'r')
    string = open_file.read()
    open_file.close()
    return string


def total_numbers(string):
    all_nums = NUMBER_REGEX.findall(string)
    return sum(int(num) for num in all_nums)


def sum_traverse_dict(elem, exclude_values):
    acc = 0
    for key, value in elem.items():
        if value in exclude_values:
            return 0
        acc += sum_traverse_element(value, exclude_values)
        acc += sum_traverse_element(key, exclude_values)
    return acc


def sum_traverse_list(elem, excluded_values):
    return sum(sum_traverse_element(element, excluded_values) for element in elem)


def sum_traverse_element(elem, exclude_values=None):
    exclude_values = exclude_values if exclude_values else []
    acc = 0
    try:
        return int(elem)
    except (ValueError, TypeError):
        pass
    try:
        acc += sum_traverse_dict(elem, exclude_values)
    except AttributeError:
        if not isinstance(elem, str):
            acc += sum_traverse_list(elem, exclude_values)

    return acc


def total_numbers_without_reds(string):
    data = json.loads(string)
    return sum_traverse_element(data, ['red'])

if __name__ == '__main__':
    string_in = get_input(os.path.dirname(__file__) + '/input/day_12.input')
    print(total_numbers(string_in))
    string_in = get_input(os.path.dirname(__file__) + '/input/day_12.input')
    print(total_numbers_without_reds(string_in))