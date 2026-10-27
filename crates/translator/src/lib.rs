//! The front end of MappingLoom: [`api::translate`] turns a mapping document
//! and its request options into a plan or an error report, for the CLI and
//! the Java, Python and Node.js bindings alike.

pub mod api;
pub mod report;
pub mod request;
mod rml;
mod shexml;

#[cfg(feature = "jni")]
mod java;

#[cfg(feature = "neon")]
mod nodejs;

#[cfg(feature = "pyo3")]
mod python;
