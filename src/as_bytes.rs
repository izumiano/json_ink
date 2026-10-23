#![allow(unused)]

pub trait AsBytes {
	fn as_bytes(&self) -> &[u8];
}

impl AsBytes for &[u8] {
	fn as_bytes(&self) -> &[u8] {
		self
	}
}

impl AsBytes for &mut [u8] {
	fn as_bytes(&self) -> &[u8] {
		self
	}
}

impl<const N: usize> AsBytes for [u8; N] {
	fn as_bytes(&self) -> &[u8] {
		self
	}
}

impl AsBytes for str {
	fn as_bytes(&self) -> &[u8] {
		str::as_bytes(self)
	}
}

impl AsBytes for &str {
	fn as_bytes(&self) -> &[u8] {
		str::as_bytes(self)
	}
}

impl AsBytes for String {
	fn as_bytes(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl AsBytes for Vec<u8> {
	fn as_bytes(&self) -> &[u8] {
		self
	}
}

//

#[allow(unused)]
pub trait AsBytesMut {
	fn as_bytes_mut(&mut self) -> &mut [u8];
}

impl AsBytesMut for &mut [u8] {
	fn as_bytes_mut(&mut self) -> &mut [u8] {
		self
	}
}

impl<const N: usize> AsBytesMut for [u8; N] {
	fn as_bytes_mut(&mut self) -> &mut [u8] {
		self
	}
}

impl AsBytesMut for &mut str {
	fn as_bytes_mut(&mut self) -> &mut [u8] {
		unsafe { str::as_bytes_mut(self) }
	}
}

impl AsBytesMut for &mut String {
	fn as_bytes_mut(&mut self) -> &mut [u8] {
		unsafe { str::as_bytes_mut(self) }
	}
}

//

pub trait CollectBytes {
	fn collect_bytes(self) -> Vec<u8>;
}

impl CollectBytes for &[char] {
	fn collect_bytes(self) -> Vec<u8> {
		self
			.iter()
			.flat_map(|char| char.to_string().as_bytes().to_vec())
			.collect::<Vec<_>>()
	}
}

impl CollectBytes for char {
	fn collect_bytes(self) -> Vec<u8> {
		self.to_string().into_bytes()
	}
}
