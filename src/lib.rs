//! # vecio
//!
//! Vector IO operations, commonly referred to as scatter/gather IO.

#![cfg_attr(test, deny(warnings))]

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
