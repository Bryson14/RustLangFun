import myrustlib  # <-- Import the Rust implemented module (myrustlib.so)
import re
import string
import random


def __bootstrap__():
    global __bootstrap__, __loader__, __file__
    import sys
    import pkg_resources
    import imp
    __file__ = pkg_resources.resource_filename(__name__, 'myrustlib.so')
    __loader__ = None
    del __bootstrap__, __loader__
    imp.load_dynamic(__name__, __file__)


__bootstrap__()


def count_doubles(val):
    """Count repeated pair of chars ins a string"""
    total = 0
    for c1, c2 in zip(val, val[1:]):
        if c1 == c2:
            total += 1
    return total


double_re = re.compile(r'(?=(.)\1)')


def count_doubles_regex(val):
    return len(double_re.findall(val))


val = ''.join(random.choice(string.ascii_letters) for i in range(1000000))


def test_pure_python(benchmark):
    benchmark(count_doubles, val)


def test_regex(benchmark):
    benchmark(count_doubles_regex, val)


def test_rust(benchmark):  # <-- Benchmark the Rust version
    benchmark(myrustlib.count_doubles, val)


print("Hello")
with open("data.txt", "w+") as f:
    f.write("help")
