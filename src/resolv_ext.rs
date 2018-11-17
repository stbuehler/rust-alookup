use resolv::{
	error::Error,
	record::{
		RecordData,
		A,
		AAAA,
	},
	Record,
	Response,
	Section,
};

use std::{
	fmt,
	marker::PhantomData,
	net::{
		IpAddr,
		Ipv4Addr,
		Ipv6Addr,
	},
};

mod seal {
	pub trait ResponseSealed {}
}

/// Extend `resolv::Response`.
pub trait ResponseExt: seal::ResponseSealed {
	fn records<T: RecordData>(&mut self, section: Section) -> Records<T>;
}
impl seal::ResponseSealed for Response {}
impl ResponseExt for Response {
	fn records<T: RecordData>(&mut self, section: Section) -> Records<T> {
		let count = self.get_section_count(section);
		Records {
			response: self,
			section,
			current: 0,
			end: count,
			_marker: PhantomData,
		}
	}
}

/// Iterate over the records of type `T` of a section
pub struct Records<'a, T: RecordData + 'a> {
	response: &'a mut Response,
	section: Section,
	current: usize,
	end: usize,
	_marker: PhantomData<&'a T>,
}

impl<'a, T: RecordData> Iterator for Records<'a, T> {
	type Item = Result<Record<T>, Error>;

	fn next(&mut self) -> Option<Self::Item> {
		while self.current < self.end {
			let index = self.current;
			self.current += 1;
			match self.response.get_record(self.section, index) {
				Err(Error::WrongRRType) => continue,
				v => return Some(v),
			}
		}
		None
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(0, Some(self.end - self.current))
	}
}

impl<'a, T: RecordData> DoubleEndedIterator for Records<'a, T> {
	fn next_back(&mut self) -> Option<Self::Item> {
		while self.current < self.end {
			self.end -= 1;
			let index = self.end;
			match self.response.get_record(self.section, index) {
				Err(Error::WrongRRType) => continue,
				v => return Some(v),
			}
		}
		None
	}
}

/// Trait for all record types that contain a single IP address as data.
pub trait AddressRecord: RecordData {
	type Address: Into<IpAddr> + fmt::Display + fmt::Debug;

	fn address(&self) -> Self::Address;
}

impl AddressRecord for A {
	type Address = Ipv4Addr;

	fn address(&self) -> Ipv4Addr {
		self.address
	}
}

impl AddressRecord for AAAA {
	type Address = Ipv6Addr;

	fn address(&self) -> Ipv6Addr {
		self.address
	}
}
