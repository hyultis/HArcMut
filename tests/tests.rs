#![allow(non_snake_case)]

use std::thread;
use HArcMut::HArcMut;

#[test]
fn update() {
	let testdefault = 42;
	let ham = HArcMut::new(testdefault);
	{
		// this is a RAII guard, the drop is needed for repercuting value into readonly part
		let mut value = ham.get_mut();
		*value = 43;
		
		// if you want to not deal with the drop, can use update() instead :
		// ```
		// ham.update(|value|{
		// 	*value = 43;
		// });
		// ```
	}
	assert_eq!(*ham.get(), 43);
}

#[test]
fn threadUpdate() {
	let testdefault = 42;
	let ham = HArcMut::new(testdefault);
	let mut threadJoin = Vec::new();
	for _ in 0..10
	{
		let hamThread = ham.clone();
		threadJoin.push(thread::spawn(move || {
			let mut value = hamThread.get_mut();
			*value += 1;
		}));
	}
	
	for x in threadJoin {
		x.join().expect("Thread join impossible");
	}
	
	assert_eq!(*ham.get(), 52);
}
