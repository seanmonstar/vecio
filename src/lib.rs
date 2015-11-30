//! # vecio
//!
//! Vector IO operations, commonly referred to as scatter/gather IO.
//!
//! # Example
//!
//! ```rust
//! use std::fs::File;
//! use vecio::Rawv;
//!
//! File::create("/dev/null").unwrap()
//!     .writev(&[b"foo", b"bar"]).unwrap();
use std::io;

mod sys;

pub trait Writev {
    fn writev(&mut self, buffers: &[&[u8]]) -> io::Result<usize>;
}

pub trait Readv {
    fn readv(&mut self, buffers: &[&mut [u8]]) -> io::Result<usize>;
}

pub trait Rawv {
    fn readv(&mut self, buffers: &[&mut [u8]]) -> io::Result<usize>;
    fn writev(&mut self, buffers: &[&[u8]]) -> io::Result<usize>;
}
