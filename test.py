#!/usr/bin/env python3

import sys, ctypes
from ctypes import POINTER, c_uint32, c_size_t

# Assuming this "test.py" file is in rust project root
testLib = ctypes.cdll.LoadLibrary("target/debug/test.dll")

# 1. Simple example
print(testLib.add(5, 3))

# 2. Advance example: Pass array from Python to Rust
#
# Todo: Do we really need these?
# testLib.sum_of_even.argtypes = (POINTER(c_uint32), c_size_t)
# testLib.sum_of_even.restype = ctypes.c_uint32

def sum_of_even(numbers):
    buf_type = c_uint32 * len(numbers)
    buf = buf_type(*numbers)
    return testLib.sum_of_even(buf, len(numbers))

print(sum_of_even([1,2,3,4]))