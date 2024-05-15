use std::sync::Arc;
use arc_swap::ArcSwap;
use crate::holderShared::HolderShared;

pub struct HolderLocal<T>
	where T: Clone
{
	pub version: ArcSwap<u64>,
	pub Data: ArcSwap<T>,
}

impl<T> HolderLocal<T>
	where T: Clone
{
	pub fn new(data: T) -> Self
	{
		return Self {
			version: ArcSwap::new(Arc::new(0)),
			Data: ArcSwap::new(Arc::new(data)),
		};
	}
	
	pub fn isOlderThan(&self, other: u64) -> bool
	{
		**self.version.load() < other
	}
	
	pub fn updateIfOlder(&self, shared: &HolderShared<T>)
	{
		let time = shared.version.load();
		if ( self.isOlderThan(**time))
		{
			let content = shared.Data.lock();
			self.version.swap(time.clone());
			self.Data.swap(Arc::new(content.clone()));
		}
	}
}

impl<T> Clone for HolderLocal<T>
	where T: Clone
{
	fn clone(&self) -> Self {
		return Self {
			version: ArcSwap::new(self.version.load().clone()),
			Data: ArcSwap::new(self.Data.load().clone()),
		};
	}
}
