#![allow(non_snake_case)]
#![allow(unused_parens)]

mod guard;
mod holder;

use std::sync::Arc;
use arc_swap::{ArcSwap, Guard};
use crate::guard::GuardMut;
use crate::holder::{HolderLocal, HolderShared};

/// HArcMut : Hyultis Arc Mut
/// store a content inside a Arc<RwLock<>> to be a mutable between thread
/// use a cloned "local" version of the content, for faster/simpler access
pub struct HArcMut<T>
	where T: Clone
{
	_shared: Arc<HolderShared<T>>,
	_local: HolderLocal<T>,
	_wantDrop: Arc<ArcSwap<bool>>
}

impl<T> HArcMut<T>
	where T: Clone
{
	pub fn new(data: T) -> Self
	{
		return HArcMut
		{
			_local: HolderLocal::new(data.clone()),
			_shared: Arc::new(HolderShared::new(data)),
			_wantDrop: Arc::new(ArcSwap::new(Arc::new(false))),
		};
	}
	
	/// get readonly content
	pub fn get(&self) -> Guard<Arc<T>>
	{
		self._local.updateIfOlder(&*self._shared);
		self._local.Data.load()
	}
	
	/// update local and shared content via a guard
	/// and readonly part by cloning on drop (*beware*: dropping guard is important to get shared and local updated and sync)
	pub fn get_mut(&self) -> GuardMut<T>
	{
		GuardMut {
			context: self,
			guarded: self._shared.Data.lock(),
		}
	}
	
	/// update local and shared content (and readonly part by cloning)
	/// this is a bit slower than get_mut, but dont need a drop.
	/// note : I is simply ignored (QOL)
	pub fn update<I>(&self, mut fnUpdate: impl FnMut(&mut T) -> I)
	{
		let (cloned,time) = {
			let mut tmp = self._shared.Data.lock();
			fnUpdate(&mut tmp);
			(tmp.clone(),self._shared.updateTime())
		};
		
		self._local.TimeUpdate.swap(Arc::new(time));
		self._local.Data.swap(Arc::new(cloned));
	}
	
	/// if closure return "true" update local part by cloning the updated shared content
	/// *beware if you update the &mut, but returning false* : shared and local data will be desync
	pub fn updateIf(&self, mut fnUpdate: impl FnMut(&mut T) -> bool)
	{
		let (cloned,time) = {
			let mut tmp = self._shared.Data.lock();
			if(!fnUpdate(&mut tmp))
			{
				return;
			}
			(tmp.clone(),self._shared.updateTime())
		};
		
		self._local.TimeUpdate.swap(Arc::new(time));
		self._local.Data.swap(Arc::new(cloned));
	}

	/// must be regulary manually checked
	/// if true, the local storage must drop this local instance
	pub fn isWantDrop(&self) -> bool
	{
		return **self._wantDrop.load();
	}

	/// used to set the state of shared intance to "Want drop"
	/// and normally be used juste before dropping the local instance
	pub fn setDrop(&self)
	{
		self._wantDrop.swap(Arc::new(true));
	}
	
	//////////////////// PRIVATE /////////////////
	
	fn update_internal(&self, cloned : T)
	{
		let time = self._shared.updateTime();
		self._local.TimeUpdate.swap(Arc::new(time));
		self._local.Data.swap(Arc::new(cloned));
	}
}

impl<T> Clone for HArcMut<T>
	where T: Clone
{
	fn clone(&self) -> Self {
		return HArcMut {
			_shared: self._shared.clone(),
			_local: self._local.clone(),
			_wantDrop: self._wantDrop.clone(),
		};
	}
}


/* Hope, one day ?
impl<T> HArcMut<T>
	where T: Clone + Any
{
	pub fn get_as<I: 'static>(&self) -> Option<RwLockReadGuard<'_,RawRwLock,I>>
	{
		self._local.updateIfOlder(self._shared.as_ref());
		let tmp = &self._local.Data as &dyn Any;
		return match tmp.downcast_ref::<RwLock<I>>() {
			None => None,
			Some(x) => {
				Some(x.read())
			}
		};
	}
	
	pub fn get_mut_as<I>(&self) -> Option<Guard<'_, RwLockWriteGuard<'static,I>>>
		where I: 'static + Clone,
		RwLockWriteGuard<'static,I> : Clone
	{
		let tmp = &self._shared.Data as &dyn Any;
		return match tmp.downcast_ref::<RwLock<I>>() {
			None => None,
			Some(x) => {
				Guard::<I>{
					context: self,
					guarded: x.write()
				}
			}
		};
	}
}
*/
