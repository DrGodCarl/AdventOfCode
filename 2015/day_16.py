import os
import re

SUE_NAME_REGEX = re.compile('(\w+ \d+):')
KEY_VALUE_REGEX = re.compile('(\w+: \d+)')


def get_sue_map(line):
    sue = SUE_NAME_REGEX.findall(line)[0]
    acc = {}
    for thing in KEY_VALUE_REGEX.findall(line):
        key, value = thing.split(': ')
        acc[key] = int(value)
    return sue, acc


def get_sue_maps(in_file):
    acc = {}
    with open(in_file, 'r') as open_file:
        for line in open_file:
            key, value = get_sue_map(line)
            acc[key] = value
    return acc


def get_result_maps(in_file):
    acc = {}
    with open(in_file, 'r') as open_file:
        for line in open_file:
            key, value = line.split(': ')
            acc[key] = int(value)
    return acc


def discern_sue(complete_sue, partial_sues):
    def merge_two_dicts(dict_1, dict_2):
        dict_3 = dict_1.copy()
        dict_3.update(dict_2)
        return dict_3
    return [sue for sue in partial_sues if complete_sue == merge_two_dicts(complete_sue, partial_sues[sue])][0]


def discern_sue_2(complete_sue, partial_sues):
    def evaluate_sue(partial_sue):
        for key in partial_sue:
            guess_val = partial_sue[key]
            complete_val = complete_sue[key]
            if key in ['cats', 'trees']:
                if guess_val <= complete_val:
                    return False
            elif key in ['pomeranians', 'goldfish']:
                if guess_val >= complete_val:
                    return False
            elif guess_val != complete_val:
                return False
        return True
    return [sue for sue in partial_sues if evaluate_sue(partial_sues[sue])][0]

if __name__ == '__main__':
    result_map = get_result_maps(os.path.dirname(__file__) + '/input/day_16.2.input')
    sue_maps = get_sue_maps(os.path.dirname(__file__) + '/input/day_16.1.input')
    print(discern_sue(result_map, sue_maps))
    print(discern_sue_2(result_map, sue_maps))