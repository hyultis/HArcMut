#![allow(non_snake_case)]
#![allow(unused_parens)]

use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use parking_lot::{RawRwLock, RwLock, RwLockUpgradableReadGuard, RwLockWriteGuard};
use parking_lot::lock_api::RwLockReadGuard;

/// HArcMut : Hyultis Arc Mut
/// store a content inside a Arc<RwLock<>> to be a mutable between thread
/// use a cloned "local" version of the content, for faster/simpler access
pub struct HArcMut<T>
{
	_sharedData: Arc<RwLock<T>>,
	_sharedLastUpdate: Arc<RwLock<u128>>,
	_localLastUpdate: RwLock<u128>,
	_localData: RwLock<T>,
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
			_localData: RwLock::new(data),
		};
	}
	
	/// get readonly content
	pub fn get(&self) -> RwLockReadGuard<'_, RawRwLock, T>
	{
		let otherTime = *self._sharedLastUpdate.read();
		if ( {*self._localLastUpdate.read()} < otherTime)
		{
			*self._localLastUpdate.write() = otherTime;
			*self._localData.write() = self._sharedData.write().clone();
		}
		return self._localData.read();
	}
	
	/// update content via a guard
	/// and readonly part by cloning on drop (beware, the drop is important to get updated data on get)
	pub fn get_mut(&self) -> Guard<'_, T>
	{
		//let tmp = self._sharedData.write();
		Guard{
			context: self,
			guarded: self._sharedData.write()
		}
	}
	
	/// update content (and readonly part by cloning)
	/// this is a bit slower than get_mut, but dont need a drop.
	/// note : I is simply ignored (QOL)
	pub fn update<I>(&self, mut fnUpdate: impl FnMut(&mut T) -> I)
	{
		let tmp = &mut self._sharedData.write();
		let timetmp = getTime();
		fnUpdate(tmp);
		*self._sharedLastUpdate.write() = timetmp;
		*self._localLastUpdate.write() = timetmp;
		*self._localData.write() = tmp.clone();
	}
	
	//////////////////// PRIVATE /////////////////
	
	fn update_internal(&self, tmp : &T)
	{
		let timetmp = getTime();
		*self._sharedLastUpdate.write() = timetmp;
		*self._localLastUpdate.write() = timetmp;
		*self._localData.write() = tmp.clone();
	}
}

impl<T> Clone for HArcMut<T>
	where T: Clone
{
	fn clone(&self) -> Self {
		return HArcMut {
			_sharedData: self._sharedData.clone(),
			_sharedLastUpdate: self._sharedLastUpdate.clone(),
			_localLastUpdate: RwLock::new(*self._sharedLastUpdate.read()),
			_localData: RwLock::new(self._localData.read().clone()),
		};
	}
}

/// this need to be faster (if possible) and monotonic, for futur update
fn getTime() -> u128
{
	return SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
}

// RAII guard
pub struct Guard<'a,T>
	where T: Clone
{
	context: &'a HArcMut<T>,
	guarded: RwLockWriteGuard<'a,T>
}

impl<'a,T> Deref for Guard<'a,T>
	where T: Clone
{
	type Target = T;
	
	fn deref(&self) -> &Self::Target {
		self.guarded.deref()
	}
}

impl<T> DerefMut for Guard<'_,T>
	where T: Clone
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.guarded.deref_mut()
	}
}

impl<T> Drop for Guard<'_,T>
	where T: Clone
{
	fn drop(&mut self) {
		self.context.update_internal(&self.guarded);
	}
}
