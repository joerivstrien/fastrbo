# Fast-RBO -- fast rank biased overlap

## About
This is a fast rust-based implementation of the <ins>non-extrapolated</ins> rank biased overlap metric, for python.

    Webber, William, Alistair Moffat, and Justin Zobel. "A similarity measure for indefinite rankings." ACM Transactions on Information Systems (TOIS) 28.4 (2010): 1-38.

## Installation

**dependencies**

- [cargo/rust](https://www.rust-lang.org/) 
- [python >= 3.7](https://www.python.org/)  
- [pyO3](https://github.com/PyO3/pyo3)
- [maturin](https://github.com/PyO3/maturin)

**to install this package for use with python:**

    git clone https://github.com/joerivstrien/fastrbo
    cd fastrbo
    maturin develop --release

## Usage
    from fast_rbo import rank_biased_overlap
    result = rank_biased_overlap(search_depth,p,left,right)

- <ins>search_depth</ins>: the number of ranks to use for computation of rbo. Set to length >= longest list to include all ranks
- <ins>p</ins>: value between 0 and 1. the rbo p-parameter, which determines the "top-heaviness" of the rank biased overlap. Lower values result in more emphasis
on the top of the ranked lists
- <ins>left/right</ins>: the ranked lists to compute the rank biased overlap for


## Issues
If you have questions or encounter any problems or bugs, please report this in the [issue channel](https://github.com/joerivstrien/fastrbo/issues).


## Licence

Copyright 2022 Joeri van Strien 

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
