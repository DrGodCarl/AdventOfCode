import os
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
    places, distance = line.split(' = ')
    distance = int(distance)
    place_1, place_2 = places.split(' to ')
    insert_place_into_map(place_1, place_2, distance)
    insert_place_into_map(place_2, place_1, distance)


def populate_map(in_file):
    open_file = open(in_file, 'r')
    for line in open_file:
        insert_into_map(line)
    open_file.close()


def distance_for_permutation(perm):
    total = 0
    first_stop = perm[0]
    for second_stop in perm:
        if second_stop is first_stop:
            continue
        total += UNIVERSAL_MAP[first_stop][second_stop]
        first_stop = second_stop
    return total


def find_shortest_distance():
    locations = UNIVERSAL_MAP.keys()
    all_perms = itertools.permutations(locations)
    minimum = 100000000000
    for perm in all_perms:
        distance = distance_for_permutation(perm)
        if distance < minimum:
            minimum = distance
    return minimum


def find_longest_distance():
    locations = UNIVERSAL_MAP.keys()
    all_perms = itertools.permutations(locations)
    maximum = 0
    for perm in all_perms:
        distance = distance_for_permutation(perm)
        if distance > maximum:
            maximum = distance
    return maximum


if __name__ == '__main__':
    populate_map(os.path.dirname(__file__) + '/input/day_9.input')
    print(find_shortest_distance())
    print(find_longest_distance())