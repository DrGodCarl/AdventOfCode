import os
import re

DURATION = 2503


class Reindeer(object):
    def __init__(self, name, speed, duration, rest_duration):
        self.name = name
        self.speed = speed
        self.duration = duration
        self.rest_duration = rest_duration
        self.distance_run = 0
        self.remaining_duration = self.duration
        self.remaining_rest_duration = 0
        self.score = 0

    def distance_run_after(self, time):
        iterations = int(time / (self.duration + self.rest_duration))
        remeindeer = time % (self.duration + self.rest_duration)
        distance = iterations * self.speed * self.duration
        distance += min(remeindeer, self.duration) * self.speed
        return distance

    def _rest(self):
        self.remaining_rest_duration -= 1
        if not self.remaining_rest_duration:
            self.remaining_duration = self.duration

    def _run(self):
        self.remaining_duration -= 1
        self.distance_run += self.speed
        if not self.remaining_duration:
            self.remaining_rest_duration = self.rest_duration

    def run_for_a_second(self):
        if self.remaining_rest_duration:
            self._rest()
            return

        if self.remaining_duration:
            self._run()
            return

    def award_point(self):
        self.score += 1

    def __str__(self):
        return self.name


def reindeer_from_line(line):
    line = line.strip(' seconds.\n')
    name, speed, duration, rest_duration = re.split(r' can fly | km/s for | seconds, but then must rest for ', line)
    return Reindeer(name, int(speed), int(duration), int(rest_duration))


def create_reindeer(in_file):
    acc = []
    open_file = open(in_file, 'r')
    for line in open_file:
        acc.append(reindeer_from_line(line))
    open_file.close()
    return acc


def race_reindeer_for_duration(reindeer, duration):
    distance = 0
    for deer in reindeer:
        new_dist = deer.distance_run_after(duration)
        distance = max(distance, new_dist)
    return distance


def score_reindeer_for_duration(reindeer, duration):
    distance = 0
    for t in range(duration):
        for deer in reindeer:
            deer.run_for_a_second()
            distance = max(distance, deer.distance_run)
        _ = [deer.award_point() for deer in reindeer if deer.distance_run == distance]
    return max(*[deer.score for deer in reindeer])


if __name__ == '__main__':
    reindeer_in = create_reindeer(os.path.dirname(__file__) + '/input/day_14.input')
    print(race_reindeer_for_duration(reindeer_in, DURATION))
    print(score_reindeer_for_duration(reindeer_in, DURATION))
