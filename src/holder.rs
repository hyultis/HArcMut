use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use arc_swap::ArcSwap;
use parking_lot::Mutex;

pub struct HolderLocal<T>
	where T: Clone
{
	pub TimeUpdate: ArcSwap<u128>,
	pub Data: ArcSwap<T>,
}

pub struct HolderShared<T>
	where T: Clone
{
	pub TimeUpdate: ArcSwap<u128>,
	pub Data: Mutex<T>,
}

impl<T> HolderShared<T>
	where T: Clone
{
	pub fn new(data: T) -> Self
	{
		return Self {
			TimeUpdate: ArcSwap::new(Arc::new(getTime())),
			Data: Mutex::new(data),
		};
	}
	
	pub fn updateTime(&self) -> u128
	{
		let tmp = getTime();
		self.TimeUpdate.swap(Arc::new(tmp));
		return tmp;
	}
}

impl<T> HolderLocal<T>
	where T: Clone
{
	pub fn new(data: T) -> Self
	{
		return Self {
			TimeUpdate: ArcSwap::new(Arc::new(getTime())),
			Data: ArcSwap::new(Arc::new(data)),
		};
	}
	
	pub fn isOlderThan(&self, other: u128) -> bool
	{
		**self.TimeUpdate.load() < other
	}
	
	pub fn updateIfOlder(&self, shared: &HolderShared<T>)
	{
		let time = shared.TimeUpdate.load();
		if ( self.isOlderThan(**time))
		{
			self.TimeUpdate.swap(time.clone());
			self.Data.swap(Arc::new(shared.Data.lock().clone()));
		}
	}
}

impl<T> Clone for HolderLocal<T>
	where T: Clone
{
	fn clone(&self) -> Self {
		return Self {
			TimeUpdate: ArcSwap::new(self.TimeUpdate.load().clone()),
			Data: ArcSwap::new(self.Data.load().clone()),
		};
	}
}

fn getTime() -> u128
{
	SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos()
}
