
use std::io;

pub trait ResourceLoader<T> {
	fn load_resource(&self, key: &str) -> io::Result<Box<[T]>>;
}

mod filesystem {
	use std::io;
	use std::io::Read;
	use std::fs;
	use std::path;

	struct ResourceLoader {
		roots: Box<[path::PathBuf]>,
	}

	struct ResourceLoaderBuilder {
		roots: Vec<path::PathBuf>,
	}

	impl ResourceLoaderBuilder {
		pub fn new() -> Self {
			ResourceLoaderBuilder { roots: Vec::new() }
		}

		pub fn add(&mut self, root: &path::Path) -> &mut Self {
			self.roots.push(root.to_owned());
			self
		}

		// consumes the builder
		pub fn build(mut self) -> ResourceLoader {
			ResourceLoader { roots: self.roots.into_boxed_slice() }
		}
	}

	impl super::ResourceLoader<u8> for ResourceLoader {
		fn load_resource(&self, key: &str) -> io::Result<Box<[u8]>> {

			// swallow the file whole into a buffer
			fn load_from_path(path: &path::Path) -> io::Result<Box<[u8]>> {
				fs::File::open(path).and_then(|mut f| {
					let mut buf = Vec::new();
					f.read_to_end(&mut buf).map(|_| buf.into_boxed_slice())
				})
				// 		Rust idiom
				// 		let mut f = try!(fs::File::open(path.as_path()));
				// 		let mut buf = Vec::new();
				// 		try!(f.read_to_end(&mut buf));
				// 		Ok(buf.into_boxed_slice())
			}

			// look for the first file which exists
			match &self.roots.iter().find(|&r| {
				let mut path = path::PathBuf::from(r);
				path.push(key);
				path.exists() && path.is_file()
			}) {
				// and then either read it
				&Some(path) => load_from_path(path),
				// or give up
				&None => Err(io::Error::new(io::ErrorKind::Other, "Resource not found in path")),
			}
		}
	}
}
