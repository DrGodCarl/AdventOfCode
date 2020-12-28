import re

ABBA_IN_BRACKETS = r'\[\w*(\w)(\w)\2\1\w*\]'
AAAA_IN_BRACKETS = r'\[\w*(\w)\1\1\1\w*\]'
ABBA_MATCH = r'(\w)(\w)\2\1'
AAAA_MATCH = r'(\w)\1\1\1'

ABA_FIRST = r'(\w)(\w)\1\w*\[(\w*\]\w*\[)*\w*\2\1\2'
BAB_FIRST = r'(\w)(\w)\1\w*\](\w*\[\w*\])*\w*\2\1\2'


def tls_regex_math(text):
    total_match = len(re.findall(ABBA_MATCH, text))
    false_match = len(re.findall(AAAA_MATCH, text))
    negate_match = len(re.findall(ABBA_IN_BRACKETS, text))
    false_negate_match = len(re.findall(AAAA_IN_BRACKETS, text))
    return (total_match - false_match) > 0 and negate_match - false_negate_match == 0


def ssl_regex_math(text):
    aba_first = len(re.findall(ABA_FIRST, text))
    bab_first = len(re.findall(BAB_FIRST, text))
    return aba_first + bab_first


def get_text():
    with open('../../resources/day7.txt', 'r') as fp:
        for row in fp:
            yield row.strip()


def run_1():
    return len([text for text in get_text() if tls_regex_math(text)])


def run_2():
    return len([text for text in get_text() if ssl_regex_math(text)])

if __name__ == '__main__':
    print run_1()
    print run_2()
