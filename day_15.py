import os
import itertools
from numpy import matlib
from operator import mul
from functools import reduce


def multipliers_from_line(line):
    _, _, capacity, _, durability, _, flavor, _, texture, _, calories = line.split()
    return (int(capacity.strip(',')),
            int(durability.strip(',')),
            int(flavor.strip(',')),
            int(texture.strip(',')),
            int(calories.strip(',')))


def get_ingredients(in_file):
    acc = []
    open_file = open(in_file, 'r')
    for line in open_file:
        acc.append(multipliers_from_line(line))
    open_file.close()
    return acc


def generate_valid_volume_matrices(volume, size):
    all_numbers = list(range(volume))
    all_combos = itertools.combinations_with_replacement(all_numbers, size)
    all_combos = [combo for combo in all_combos if sum(combo) == volume]
    for i in range(len(all_combos)):
        all_combos[i] = list(itertools.permutations(all_combos[i], size))
    return [item for sublist in all_combos for item in sublist]


def calculate_cookie_score(volumes, ingredient_multipliers):
    totals = [0]*len(ingredient_multipliers[0])
    for i in range(len(volumes)):
        volume_of_current_ingredient = volumes[i]
        current_ingredient = ingredient_multipliers[i]
        for j in range(len(current_ingredient)):
            totals[j] += volume_of_current_ingredient * current_ingredient[j]
    totals = [max(num, 0) for num in totals]
    return reduce(mul, totals)


def determine_ideal_cookie(ingredient_multipliers, tsp, ignore_calories=True):
    if ignore_calories:
        ingredient_multipliers = [ingredient_multiplier[:-1] for ingredient_multiplier in ingredient_multipliers]
    potential_volumes = generate_valid_volume_matrices(tsp, len(ingredient_multipliers[0]))
    best = 0
    for potential_volume in potential_volumes:
        old_best = best
        best = max(best, calculate_cookie_score(potential_volume, ingredient_multipliers))
        if best != old_best:
            print(potential_volume)
    return best


if __name__ == '__main__':
    ingredients = get_ingredients(os.path.dirname(__file__) + '/input/day_15.input')
    print(determine_ideal_cookie(ingredients, 100))

