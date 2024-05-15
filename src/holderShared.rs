use arc_swap::ArcSwap;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct HolderShared<T>
	where T: Clone
{
	pub version: ArcSwap<u64>,
	pub Data: Mutex<T>,
}

impl<T> HolderShared<T>
	where T: Clone
{
	pub fn new(data: T) -> Self
	{
		return Self {
			version: ArcSwap::new(Arc::new(0)),
			Data: Mutex::new(data),
		};
	}
	
	pub fn updateVersion(&self) -> Arc<u64>
	{
		let old = **self.version.load();
		let tmp = Arc::new(old + 1);
		self.version.swap(tmp.clone());
		return tmp;
	}
}
