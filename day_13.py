
import os
import re
import itertools

UNIVERSAL_MAP = {}


def insert_place_into_map(place_1, place_2, distance):
    try:
        sub_map = UNIVERSAL_MAP[place_1]
    except KeyError:
        sub_map = {}
        UNIVERSAL_MAP[place_1] = sub_map
    sub_map[place_2] = distance


def insert_into_map(line):
    line = line.replace('gain ', '').replace('lose ', '-').strip().strip('.')
    person_1, diff, person_2 = re.split(' would | happiness units by sitting next to ', line)
    insert_place_into_map(person_1, person_2, int(diff))


def populate_map(in_file):
    open_file = open(in_file, 'r')
    for line in open_file:
        insert_into_map(line)
    open_file.close()


def add_you_to_map():
    people = list(UNIVERSAL_MAP)
    for person in people:
        insert_place_into_map('you', person, 0)
        insert_place_into_map(person, 'you', 0)


def happiness_for_permutation(perm):
    total = 0
    first_stop = perm[0]
    for second_stop in perm:
        if second_stop is first_stop:
            continue
        total += UNIVERSAL_MAP[first_stop][second_stop]
        total += UNIVERSAL_MAP[second_stop][first_stop]
        first_stop = second_stop
    total += UNIVERSAL_MAP[first_stop][perm[0]]
    total += UNIVERSAL_MAP[perm[0]][first_stop]
    return total


def find_least_happiness():
    locations = UNIVERSAL_MAP.keys()
    all_perms = itertools.permutations(locations)
    minimum = 100000000000
    for perm in all_perms:
        distance = happiness_for_permutation(perm)
        if distance < minimum:
            minimum = distance
    return minimum


def find_greatest_happiness():
    people = UNIVERSAL_MAP.keys()
    all_perms = itertools.permutations(people, len(people))
    maximum = 0
    for perm in all_perms:
        distance = happiness_for_permutation(perm)
        if distance > maximum:
            maximum = distance
    return maximum


if __name__ == '__main__':
    populate_map(os.path.dirname(__file__) + '/input/day_13.input')
    print(find_greatest_happiness())
    add_you_to_map()
    print(find_greatest_happiness())
