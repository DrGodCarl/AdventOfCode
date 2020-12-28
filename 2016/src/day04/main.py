from collections import defaultdict


class EncryptedRoom(object):
    def __init__(self, string):
        parts = string.split('-')
        sector_checksum = parts[-1].split('[')
        self.name = '-'.join(parts[:-1])
        self.sorted_name = sorted(''.join(parts[:-1]))
        self.sector = int(sector_checksum[0])
        self.checksum = sector_checksum[1].strip(']')
        self.decrypted = ''

    def is_valid(self):
        return self.generate_checksum() == self.checksum

    def generate_checksum(self):
        count_map = defaultdict(int)
        for char in self.sorted_name:
            count_map[char] += 1
        return ''.join([pair[0] for pair in sorted(count_map.items(), key=lambda item: (-item[1], item[0]))][:5])

    def decrypt(self):
        def rotate_char():
            if char == '-':
                return ' '
            return chr((ord(char) - ord('a') + rot) % 26 + ord('a'))

        rot = self.sector % 26
        self.decrypted = ''.join([rotate_char() for char in self.name])

    def __str__(self):
        if self.decrypted:
            return self.decrypted + ' ' + str(self.sector)
        return self.name + '-' + str(self.sector) + '[' + self.checksum + ']'


def get_encrypted_rooms():
    with open('../../resources/day4.txt', 'r') as fp:
        for line in fp:
            yield EncryptedRoom(line.strip())


def sector_if_valid(room):
    return room.sector if room.is_valid() else 0


def run_1():
    return reduce(lambda i, r: i + sector_if_valid(r), get_encrypted_rooms(), 0)


def run_2():
    for room in get_encrypted_rooms():
        if not room.is_valid():
            continue
        room.decrypt()
        if 'northpole' in room.decrypted:
            return room.sector

if __name__ == '__main__':
    print run_1()
    print run_2()
