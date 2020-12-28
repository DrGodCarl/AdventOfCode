import hashlib

INPUT = 'ffykfhsq'


def is_interesting(hex_val):
    return hex_val[:5] == '00000'


def is_very_interesting(hex_val):
    try:
        return is_interesting(hex_val) and int(hex_val[5]) < 8
    except ValueError:
        return False


def is_useful_and_very_interesting(hex_val, current_guess):
    if is_very_interesting(hex_val):
        return not current_guess[int(hex_val[5])]


def run_1():
    val = 0
    result = ''
    while len(result) < 8:
        m = hashlib.md5()
        m.update(INPUT + str(val))
        hex_val = m.hexdigest()
        if is_interesting(hex_val):
            result += hex_val[5]
        val += 1
    return result


def run_2():
    val = 0
    result = [''] * 8
    count = 0
    while count < 8:
        m = hashlib.md5()
        m.update(INPUT + str(val))
        hex_val = m.hexdigest()
        if is_useful_and_very_interesting(hex_val, result):
            result[int(hex_val[5])] = hex_val[6]
            count += 1
        val += 1
    return ''.join(result)

if __name__ == '__main__':
    # print run_1()
    print run_2()
