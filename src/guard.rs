use std::ops::{Deref, DerefMut};
use parking_lot::RwLockWriteGuard;
use crate::HArcMut;

// RAII guard
pub struct Guard<'a,T>
	where T: Clone
{
	pub context: &'a HArcMut<T>,
	pub guarded: RwLockWriteGuard<'a,T>
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
