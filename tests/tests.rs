#![allow(non_snake_case)]
use std::thread;
use HArcMut::HArcMut;

#[test]
fn update() {
	let testdefault = 42;
	let ham = HArcMut::new(testdefault);
	ham.update(|i| {
		*i = 43;
	});
	assert_eq!(*ham.get(), 43);
}

#[test]
fn threadUpdate() {
	let testdefault = 42;
	let ham = HArcMut::new(testdefault);
	let mut threadJoin = Vec::new();
	for i in 0..10
	{
		let hamThread = ham.clone();
		threadJoin.push(thread::spawn(move || {
			println!("======{}", i);
			hamThread.update(|i| {
				*i += 1;
			});
		}));
	}
	
	for x in threadJoin {
		x.join().expect("Thread join impossible");
	}
	
	assert_eq!(*ham.get(), 52);
}
