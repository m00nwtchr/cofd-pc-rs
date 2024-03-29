#![feature(generic_const_exprs)]

pub mod scraper;

extern crate cofd_derive;

pub use cofd_derive::*;

pub trait VariantName {
	fn name(&self) -> &str;
}

impl<T> VariantName for Box<T>
where
	T: VariantName,
{
	fn name(&self) -> &str {
		self.as_ref().name()
	}
}

pub trait AllVariants {
	type T;
	const N: usize;
	fn all() -> [Self::T; Self::N];
}
