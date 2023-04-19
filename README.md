# HArcMut

A library that *mimic* a mutable Arc. Because it will be never possible in pure rust.

I use a shared RwLock, that permit a writable ***Data*** between thread and a local ReadOnly version for faster/simplest access.
***Data*** is synchronized on get (not on write !) to assure getting the updated version when needed.

***Data*** need to have the "Clone" trait


## Online Documentation

[Master branch](https://github.com/hyultis/HArcMut)

## Example

You can check the test as example, here : https://github.com/hyultis/HArcMut/blob/master/src/lib.rs#L99

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 licence, shall be
dual licensed as above, without any additional terms or conditions.
