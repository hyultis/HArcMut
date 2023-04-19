#![allow(non_snake_case)]
#![allow(unused_parens)]

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use arc_swap::ArcSwap;
use parking_lot::RwLock;

/// HArcMut : Hyultis Arc Mut
/// store a content inside a Arc<RwLock<>> to be a mutable between thread
/// use a cloned "local" version of the content, for faster/simpler access
pub struct HArcMut<T>
	where T: ?Sized + Clone
{
	_sharedData: Arc<RwLock<T>>,
	_sharedLastUpdate: Arc<RwLock<u128>>,
	_localLastUpdate: RwLock<u128>,
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
			_localLastUpdate: RwLock::new(time),
			_localData: ArcSwap::new(Arc::new(data)),
		};
	}
	
	pub fn get(&self) -> Arc<T>
	{
		let otherTime = *self._sharedLastUpdate.read();
		let returning ;
		if({*self._localLastUpdate.read()} < otherTime)
		{
			*self._localLastUpdate.write() = otherTime;
			returning = Arc::new(self._sharedData.write().clone());
			self._localData.swap(returning.clone());
		}
		else
		{
			returning = self._localData.load().clone();
		}
		return returning;
	}
	
	/// update content (and readonly part by cloning, so its costly)
	/// check updateIf() if you want to update "readonly" depending from the closure
	/// note : I is simply ignored (QOL)
	pub fn update<I>(&self, mut fnUpdate: impl FnMut(&mut T) -> I)
	{
		let tmp = &mut self._sharedData.write();
		let timetmp = getTime();
		fnUpdate(tmp);
		*self._sharedLastUpdate.write() = timetmp;
		*self._localLastUpdate.write() = timetmp;
		self._localData.swap(Arc::new(tmp.clone()));
	}
	
	/// like update(),
	/// but closure must return true if something changed (to update the readonly part), or false
	pub fn updateIf(&self, mut fnUpdate: impl FnMut(&mut T) -> bool)
	{
		let tmp = &mut self._sharedData.write();
		if(fnUpdate(tmp))
		{
			let timetmp = getTime();
			*self._sharedLastUpdate.write() = timetmp;
			*self._localLastUpdate.write() = timetmp;
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
			_localLastUpdate: RwLock::new(*self._sharedLastUpdate.read()),
			_localData: ArcSwap::new(Arc::new(self._sharedData.read().clone()))
		};
	}
}

/// this need to be faster (if possible) and monotonic, for futur update
fn getTime() -> u128
{
	return SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
}
