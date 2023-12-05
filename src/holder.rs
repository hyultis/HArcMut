use std::time::{SystemTime, UNIX_EPOCH};
use parking_lot::RwLock;

pub struct Holder<T>
	where T: Clone
{
	pub TimeUpdate: RwLock<u128>,
	pub Data: RwLock<T>,
}

impl<T> Holder<T>
	where T: Clone
{
	pub fn new(data: T) -> Self
	{
		return Holder {
			TimeUpdate: RwLock::new(Holder::<T>::getTime()),
			Data: RwLock::new(data),
		};
	}
	
	pub fn isOlderThan(&self, other: u128) -> bool
	{
		*self.TimeUpdate.read() < other
	}
	
	pub fn updateIfOlder(&self, shared: &Self)
	{
		let otherTime = match shared.TimeUpdate.try_read() {
			None => return,
			Some(otherTime) => *otherTime
		};
		
		if ( self.isOlderThan(otherTime))
		{
			*self.TimeUpdate.write() = otherTime;
			*self.Data.write() = shared.Data.read().clone();
		}
	}
	
	pub fn updateTime(&self) -> u128
	{
		let tmp = Holder::<T>::getTime();
		*self.TimeUpdate.write() = tmp;
		return tmp;
	}
	
	pub fn getTime() -> u128
	{
		SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos()
	}
}

impl<T> Clone for Holder<T>
	where T: Clone
{
	fn clone(&self) -> Self {
		return Holder {
			TimeUpdate: RwLock::new(*self.TimeUpdate.read()),
			Data: RwLock::new(self.Data.read().clone()),
		};
	}
}
