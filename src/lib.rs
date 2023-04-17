#![allow(non_snake_case)]
#![allow(unused_parens)]

use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use arc_swap::{ArcSwap};

/// HArcMut : Hyultis Arc Mut
/// store a content inside a Arc<RwLock<>> to be a mutable between thread
/// use a cloned "local" version of the content, for faster/simpler access
pub struct HArcMut<T>
	where T: ?Sized + Clone
{
	_sharedData: Arc<RwLock<T>>,
	_sharedLastUpdate: Arc<RwLock<u128>>,
	_localLastUPdate: RwLock<u128>,
	_localData: ArcSwap<T>
}

impl<T> HArcMut<T>
	where T: Clone
{
	pub fn new(data: T) -> Self
	{
		let time = getTime();
		return HArcMut
		{
			_sharedData: Arc::new(RwLock::new(data.clone())),
			_sharedLastUpdate: Arc::new(RwLock::new(time)),
			_localLastUPdate: RwLock::new(time),
			_localData: ArcSwap::new(Arc::new(data)),
		};
	}
	
	pub fn get(&self) -> Arc<T>
	{
		let otherTime = *self._sharedLastUpdate.read().unwrap();
		let returning ;
		if({*self._localLastUPdate.read().unwrap()} < otherTime)
		{
			*self._localLastUPdate.write().unwrap() = otherTime;
			returning = Arc::new(self._sharedData.write().unwrap().clone());
			self._localData.swap(returning.clone());
		}
		else
		{
			returning = self._localData.load().clone();
		}
		return returning;
	}
	
	/// update content (and readonly part by cloning, so its costly)
	/// check updateIf if you want to update "readonly" depending from the closure
	/// note : I is simply ignored (QOL)
	pub fn update<I>(&self, mut fnUpdate: impl FnMut(&mut T) -> I)
	{
		let tmp = &mut self._sharedData.write().unwrap();
		fnUpdate(tmp);
		let timetmp = getTime();
		*self._sharedLastUpdate.write().unwrap() = timetmp;
		*self._localLastUPdate.write().unwrap() = timetmp;
		self._localData.swap(Arc::new(tmp.clone()));
	}
	
	/// like update,
	/// but closure must return true if something changed (to update the readonly part), or false
	pub fn updateIf(&self, mut fnUpdate: impl FnMut(&mut T) -> bool)
	{
		let mut tmp = self._sharedData.write().unwrap();
		if(fnUpdate(&mut tmp))
		{
			let timetmp = getTime();
			*self._sharedLastUpdate.write().unwrap() = timetmp;
			*self._localLastUPdate.write().unwrap() = timetmp;
			self._localData.swap(Arc::new(tmp.clone()));
		}
	}
}

impl<T> Clone for HArcMut<T>
	where T: Clone
{
	fn clone(&self) -> Self {
		return HArcMut{
			_sharedData: self._sharedData.clone(),
			_sharedLastUpdate: self._sharedLastUpdate.clone(),
			_localLastUPdate: RwLock::new(*self._sharedLastUpdate.read().unwrap()),
			_localData: ArcSwap::new(Arc::new(self._sharedData.read().unwrap().clone()))
		};
	}
}

fn getTime() -> u128
{
	return SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
}


#[cfg(test)]
mod tests {
	use std::thread;
	use super::*;

    #[test]
    fn update() {
	    let testdefault = 42;
	    let ham = HArcMut::new(testdefault);
	    ham.update(|i|{
		    *i = 43;
	    });
        assert_eq!(*ham.get(), 43);
    }
	
	#[test]
	fn threadUpdate() {
		let testdefault = 42;
		let ham = HArcMut::new(testdefault);
		for _ in 0..10
		{
			let hamThread = ham.clone();
			thread::spawn(move || {
				hamThread.update(|i| {
					*i += 1;
				});
			}).join().expect("Thread join impossible");
		}
		
		assert_eq!(*ham.get(), 52);
	}
}
