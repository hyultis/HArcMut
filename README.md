# HArcMut

A library that *mimic* a mutable Arc.

I use a shared RwLock, that permit a writable ***Data*** between thread and a local ReadOnly version for faster/simplest access.
***Data*** is synchronized on get (not on write !) to assure getting the updated version when needed.

***Data*** need to have the "Clone" trait

use [Parking_lot](https://crates.io/crates/parking_lot) and [arc-swap](https://crates.io/crates/arc-swap) for sync stuff

Beware : for a safe memory clear, you need to drop each clone, everywhere.
To help with that, you need to check the return of "isWantDrop()", and removing the local instance from your storage (and doing that for each system that hold a clone)

## Online Documentation

[Master branch](https://github.com/hyultis/HArcMut)

## Example

You can check the test as example, here : https://github.com/hyultis/HArcMut/blob/master/tests/tests.rs

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 licence, shall be
dual licensed as above, without any additional terms or conditions.
