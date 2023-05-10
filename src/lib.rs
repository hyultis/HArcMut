#![allow(non_snake_case)]
#![allow(unused_parens)]

use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use parking_lot::{RawRwLock, RwLock, RwLockWriteGuard};
use parking_lot::lock_api::RwLockReadGuard;

/// HArcMut : Hyultis Arc Mut
/// store a content inside a Arc<RwLock<>> to be a mutable between thread
/// use a cloned "local" version of the content, for faster/simpler access
pub struct HArcMut<T>
	where T: Clone
{
	_sharedData: Arc<RwLock<T>>,
	_sharedLastUpdate: Arc<RwLock<u128>>,
	_sharedWantDrop: Arc<RwLock<bool>>,
	_localLastUpdate: RwLock<u128>,
	_localData: RwLock<T>,
	_localWantDrop: RwLock<bool>
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
			_sharedWantDrop: Arc::new(RwLock::new(false)),
			_localLastUpdate: RwLock::new(time),
			_localData: RwLock::new(data),
			_localWantDrop: RwLock::new(false),
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
			*self._localWantDrop.write() = self._sharedWantDrop.write().clone();
		}
		return self._localData.read();
	}
	
	/// update local and shared content via a guard
	/// and readonly part by cloning on drop (*beware*: dropping guard is important to get shared and local updated and sync)
	pub fn get_mut(&self) -> Guard<'_, T>
	{
		//let tmp = self._sharedData.write();
		Guard{
			context: self,
			guarded: self._sharedData.write()
		}
	}
	
	/// update local and shared content (and readonly part by cloning)
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
	
	/// if closure return "true" update local part by cloning the updated shared content
	/// *beware if you update the &mut, but returning false* : shared and local data will be desync
	pub fn updateIf(&self, mut fnUpdate: impl FnMut(&mut T) -> bool)
	{
		let tmp = &mut self._sharedData.write();
		if (fnUpdate(tmp))
		{
			let timetmp = getTime();
			*self._sharedLastUpdate.write() = timetmp;
			*self._localLastUpdate.write() = timetmp;
			*self._localData.write() = tmp.clone();
		}
	}

	/// must be regulary manually checked
	/// if true, the local storage must drop this local instance
	pub fn isWantDrop(&self) -> bool
	{
		let otherTime = *self._sharedLastUpdate.read();
		if ( {*self._localLastUpdate.read()} < otherTime)
		{
			*self._localLastUpdate.write() = otherTime;
			*self._localData.write() = self._sharedData.write().clone();
			*self._localWantDrop.write() = self._sharedWantDrop.write().clone();
		}
		return *self._localWantDrop.read();
	}

	/// used to set the state of shared intance to "Want drop"
	/// and normally be used juste before dropping the local instance
	pub fn setDrop(&self)
	{
		let timetmp = getTime();
		*self._sharedLastUpdate.write() = timetmp;
		*self._sharedWantDrop.write() = true;
	}
	
	//////////////////// PRIVATE /////////////////
	
	fn update_internal(&self, tmp : T)
	{
		let timetmp = getTime();
		*self._sharedLastUpdate.write() = timetmp;
		*self._localLastUpdate.write() = timetmp;
		*self._localData.write() = tmp;
	}
}

impl<T> Clone for HArcMut<T>
	where T: Clone
{
	fn clone(&self) -> Self {
		return HArcMut {
			_sharedData: self._sharedData.clone(),
			_sharedLastUpdate: self._sharedLastUpdate.clone(),
			_sharedWantDrop: self._sharedWantDrop.clone(),
			_localLastUpdate: RwLock::new(*self._sharedLastUpdate.read()),
			_localData: RwLock::new(self._localData.read().clone()),
			_localWantDrop: RwLock::new(self._sharedWantDrop.read().clone()),
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
		self.context.update_internal(self.guarded.clone()); // no way to do it without a clone ?
	}
}
