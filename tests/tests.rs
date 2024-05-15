#![allow(non_snake_case)]

use std::thread;
use HArcMut::HArcMut;

#[test]
fn update()
{
	let testdefault = 42;
	let ham = HArcMut::new(testdefault);
	{
		// this is a RAII guard, the drop is needed for reverberate value (done with a clone) into readonly part
		let mut value = ham.get_mut();
		*value = 43;

		// if you want to not deal with the drop, can use update() instead :
		// ```
		// ham.update(|value|{
		// 	*value = 43;
		// });
		// ```
	}
	assert_eq!(**ham.get(), 43);
}

#[test]
fn threadUpdate()
{
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

	for _ in 0..10
	{
		let hamThread = ham.clone();
		threadJoin.push(thread::spawn(move || {
			hamThread.update(|value| {
				*value += 1;
			});
		}));
	}

	for _ in 0..10
	{
		let hamThread = ham.clone();
		threadJoin.push(thread::spawn(move || {
			hamThread.updateIf(|value| {
				*value += 1;
				true // put in false here lead to desync, be carefull
			});
		}));
	}

	for x in threadJoin
	{
		x.join().expect("Thread join impossible");
	}

	assert_eq!(**ham.get(), 72);
}

#[test]
fn threadDrop()
{
	let testdefault = 42;
	let ham = HArcMut::new(testdefault);
	let mut storage = Vec::new();
	storage.push(ham.clone());

	thread::spawn(move || {
		ham.setDrop();
	})
	.join()
	.expect("Thread join impossible");

	storage.retain_mut(|item| !item.isWantDrop());

	assert_eq!(storage.len(), 0);
}
