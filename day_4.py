import hashlib

INPUT = 'ckczppom'


def get_hash(string):
    m = hashlib.md5()
    m.update(string.encode('UTF-8'))
    return m.hexdigest()


def verify_number(secret, number):
    md5 = get_hash(secret + str(number))
    return md5.startswith('000000')


def find_checksum(secret):
    inc = 0
    while not verify_number(secret, inc):
        inc += 1
    return inc


if __name__ == '__main__':
    print(find_checksum(INPUT))
