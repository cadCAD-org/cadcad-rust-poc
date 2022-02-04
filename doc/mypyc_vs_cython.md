
# Mypyc vs Cython

https://github.com/mypyc/mypyc  
https://cython.readthedocs.io/en/latest/src/quickstart/overview.html  

## 1. Intro

[Mypyc] compiles Python modules to C extensions. It uses standard Python type hints to generate fast code. Mypyc uses mypy to perform type checking and type inference.

[Cython] is a programming language (and also a set of tools) that makes writing C extensions for the Python language as easy as Python itself. It aims to become a superset of the [Python] language


### Key Differences:  
- Cython is a language, super set of Python. Mypyc is not.

- Cython requires using special types (e.g. cdef int x=0) in its annotations. Mypyc does not require special types, uses standard Python type hints to generate fast code. Mypyc also uses mypy to perform type checking and type inference.

- Mypyc aims to be compatible with the standard Python type annotations and still be able to optimize them. So in theory, you don’t need to modify your existing type-annotated program.
 

More:   
https://mypyc.readthedocs.io/en/latest/introduction.html#differences-from-cython


### High-level Overview of Mypyc
- Mypyc compiles a Python module (or a set of modules) to C, and compiles the generated C to a Python C extension module (or modules). You can compile only a subset of your program to C -- compiled and interpreted code can freely and transparently interact. You can also freely use any Python libraries (including C extensions) in compiled code.

- Mypyc will only make compiled code faster. To see a significant speedup, you must make sure that most of the time is spent in compiled code -- and not in libraries, for example.

More:  
https://github.com/python/mypy/blob/master/mypyc/doc/dev-intro.md#high-level-overview-of-mypyc


## 2. Simple Usages/Perf. Compared

### Simple usage of mypyc  

Summary:   
Below, we see up to 30x faster runtime for fibonacci example; but when we use a library (e.g. random) we do not see any improved performance (Note: These are initial tests, not to be taken as conclusive results)  

// test.py
```py
import random, time

# Using type hints
def fib1(n: int) -> int:
    if n <= 1:
        return n
    else:
        return fib1(n - 2) + fib1(n - 1)
# NOT using type hints
def fib2(n):
    if n <= 1:
        return n
    else:
        return fib2(n - 2) + fib2(n - 1)

# Using type hints
def sum_of_floats_1() -> float:
    sum: float = 0.0
    for i in range(100_000):
        sum += random.uniform(-10.0, 10.0)
    return (sum)

# NOT using type hints
def sum_of_floats_2():
    sum = 0.0
    for i in range(100_000):
        sum += random.uniform(-10.0, 10.0)
    return (sum)    

t0 = time.time()
fib1(32)
print("fib1: ", time.time()-t0, "sec(s)")
t0 = time.time()
fib2(32)
print("fib2: ", time.time()-t0, "sec(s)")

t0 = time.time()
sum_of_floats_1()
print("sum_of_floats_1: ", time.time()-t0, "sec(s)\n")
t0 = time.time()
sum_of_floats_2()
print("sum_of_floats_2: ", time.time()-t0, "sec(s)\n")
```

// Steps
```
// Compile with mypyc
mypyc test.py // creates: test.cp37-win_amd64.pyd, this is actually a dll file
              // See Note-1 below

// With mypc
python3 -c "import test"
fib1:  0.03400.. sec(s)   // 30x faster (using type hints)
fib2:  0.25001.. sec(s)   //  3x faster (NOT using type hints)
// Without mypyc
python3 test.py
fib1:  0.89505.. sec(s)  // (using type hints)
fib2:  0.89005.. sec(s)  // (NOT using type hints)

// With mypc
python3 -c "import test"
sum_of_floats_1:  0.03000.. sec(s)  // No big difference! (NOT using type hints)
sum_of_floats_2:  0.02800.. sec(s)  // No big difference! (using type hints)
// Without mypyc
python3 test.py
sum_of_floats_1:  0.02900.. sec(s) // (using type hints)
sum_of_floats_2:  0.02900.. sec(s) // (NOT using type hints)

```


#### Note-1
```
$ dumpbin /dependents test.cp37-win_amd64.pyd

Dump of file test.cp37-win_amd64.pyd

File Type: DLL

  Image has the following dependencies:

    api-ms-win-crt-runtime-l1-1-0.dll
    api-ms-win-crt-stdio-l1-1-0.dll
    python37.dll
    KERNEL32.dll
    api-ms-win-crt-heap-l1-1-0.dll
    api-ms-win-crt-string-l1-1-0.dll
```

### Simple usage of Сython

Summary:   
We see up to 4.5x faster runtime (it was 30x with mypyc) for fibonacci example; but when we use a library (e.g. random) we see any ~1.2x faster runtime (there was no improvement with mypyc for this case) (Note: These are initial tests, not to be taken as conclusive results)  

// test.pyx
```py
import time, random

# NOT using Cython syntax
def fib1(n):
    if n <= 1:
        return n
    else:
        return fib1(n - 2) + fib1(n - 1)

# Using Cython syntax
def fib2(int n):
    if n <= 1:
        return n
    else:
        return fib2(n - 2) + fib2(n - 1)

# NOT using Cython syntax
def sum_of_floats_1():
    sum = 0.0
    for i in range(100_000):
        sum += random.uniform(-10.0, 10.0)
    return (sum)

# Using Cython syntax
def sum_of_floats_2():
    cdef float sum = 0.0
    for i in range(100_000):
        sum += random.uniform(-10.0, 10.0)
    return (sum)    


t0 = time.time()
fib1(32)
print("fib1: ", time.time()-t0, "sec(s)")
t0 = time.time()
fib2(32)
print("fib2: ", time.time()-t0, "sec(s)")

t0 = time.time()
sum_of_floats_1()
print("sum_of_floats_1: ", time.time()-t0, "sec(s)\n")
t0 = time.time()
sum_of_floats_2()
print("sum_of_floats_2: ", time.time()-t0, "sec(s)\n")
```

// setup.py
```py
from setuptools import setup
from Cython.Build import cythonize

setup(
    ext_modules = cythonize("test.pyx")
)
```



// Steps
```
// Compile with cython
python setup.py build_ext --inplace 
    // creates: test.cp37-win_amd64.pyd, this is also a dll file
    // See Note-2 below

// With cython
python3 -c "import test"
('fib1: ', 0.28201.., 'sec(s)')  //   3x faster (NOT using cython syntax)
('fib2: ', 0.20201.., 'sec(s)')  // 4.5x faster (using cython syntax)
// Without cython
python3 test.py
fib1:  0.89105 sec(s)

// With cython
python3 -c "import test"
('sum_of_floats_1: ', 0.02400.., 'sec(s)\n') // Faster (NOT using cython syntax)
('sum_of_floats_2: ', 0.02400.., 'sec(s)\n') // Faster (using cython syntax)
// Without cython
python3 test.py
sum_of_floats_1:  0.030001.. sec(s)
```

#### Note-2
```
$ dumpbin /dependents test.cp37-win_amd64.pyd

Dump of file fib.cp37-win_amd64.pyd

File Type: DLL

  Image has the following dependencies:

    python37.dll
    KERNEL32.dll
    VCRUNTIME140.dll
    api-ms-win-crt-runtime-l1-1-0.dll
...
```