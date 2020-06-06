# Crate `ghosts_of_departed_proofs`

This crate contains implementations of a `probability` type using techniques described in [Ghosts of Departed Proofs](https://kataskeue.com/gdp.pdf). Each implementation comes with an example of what the library and caller would look like for the [binary entropy function](https://en.wikipedia.org/wiki/Binary_entropy_function), which is defined for real numbers between 0 and 1.

The crate will also have an implementation of a `merge_by` function for sorted lists, that works for any two lists that have already been sorted by the same comparator.
