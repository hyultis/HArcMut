use std::ops::{Deref, DerefMut};
use parking_lot::MutexGuard;
use crate::HArcMut;

// RAII guard
pub struct GuardMut<'a,T: 'a>
	where T: Clone
{
	pub context: &'a HArcMut<T>,
	pub guarded: MutexGuard<'a,T>
}

impl<T> Deref for GuardMut<'_,T>
	where T: Clone
{
	type Target = T;
	
	fn deref(&self) -> &Self::Target {
		&self.guarded
	}
}

impl<T> DerefMut for GuardMut<'_,T>
	where T: Clone
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.guarded
	}
}

impl<T> Drop for GuardMut<'_,T>
	where T: Clone
{
	fn drop(&mut self) {
		self.context.update_internal(self.guarded.clone()); // no way to do it without a clone ?
	}
}
