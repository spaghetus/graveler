# graveler

Submission for the Graveler Softlock Picking challenge.

## Key optimizations

* Use Rust
* Use iterators
* Don't try to count trials in real-time (this would add overhead from atomic thrashing)
* Compile for native (saves ~5s)
* Use all of your cores

## Comparison

(Performed on an AMD Ryzen 5 1600X with factory clock)

* 1 million
  * Original (cpython) - 125s
  * Original (pypy) - 12s (10x)
  * Mine - 55ms (2273x)
* 10 million
  * Original (cpython, extrapolated) - 1250s
  * Original (pypy) - 119s (10x)
  * Mine - 475ms (2631x)
* 1 billion
  * Original (cpython, extrapolated) - 125000s
  * Mine - 36s (3472x)
