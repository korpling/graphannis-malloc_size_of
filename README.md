[![Documentation link](https://docs.rs/graphannis-malloc_size_of/badge.svg)](https://docs.rs/graphannis-malloc_size_of/)

# graphannis-malloc_size_of

This is a fork of the `malloc_size_of` crate, which is part of the [Servo codebase](https://github.com/servo/servo/tree/master/components/malloc_size_of) but was not published on crates.io at the time of the fork. 
The intention of this fork is to make the functionality of the original crate available to the [graphANNIS](https://github.com/korpling/graphANNIS) corpus search library, which is published on [crates.io](https://crates.io/crates/graphannis).


The following changes have been made in comparision to the original crate:
- References and implementations to internal/unpublished Servo crates have been removed.
- All dependencies have been made optional, so if you want to use the memory size calculation for types not in the standard library you have to activate these as a feature. The following crates are available as optional dependencies/features:
    - app_units
    - cssparser
    - euclid
    - serde
    - serde_bytes
    - smallbitvec
    - smallvec
    - smartstring
    - string_cache 
    - thin-slice
    - time
    - url
    - void
    - xml5ever
