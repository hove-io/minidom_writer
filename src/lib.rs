#![deny(missing_docs)]

//! Helper for [`minidom`] to write XML.
//!
//! ## Description
//! The crate provides [`ElementWriter`] which can write an [`Element`].
//!
//! ## Example
//!
//! ```
//! use minidom::{Element, Node};
//! use minidom_writer::ElementWriter;
//! use std::io::Cursor;
//!
//! let subtag = Element::builder("subtag", "ns")
//!     .attr("id", "my_subtag")
//!     .append(Node::Text(String::from("Some text")))
//!     .build();
//! let tag = Element::builder("tag", "")
//!     .attr("id", "my_tag")
//!     .append(subtag)
//!     .build();
//! let write = Cursor::new(Vec::new());
//! let mut element_writer = ElementWriter::plain(write);
//! element_writer.write(&tag).unwrap();
//! let expected = r#"<?xml version="1.0" encoding="UTF-8"?><tag id="my_tag"><ns:subtag id="my_subtag">Some text</ns:subtag></tag>"#;
//! assert_eq!(expected, String::from_utf8(element_writer.into_inner().into_inner()).unwrap());
//! ```
//!
//! [`Element`]: ../minidom/element/struct.Element.html
//! [`ElementWriter`]: struct.ElementWriter.html
//! [`minidom`]: ../minidom/index.html

mod writer;

pub use writer::ElementWriter;

use thiserror::Error;

/// Error type for 'minidom_writer'.
#[derive(Debug, Error)]
pub enum Error {
    /// Error returned by [`quick-xml`] when trying to write an XML tag.
    /// See [`quick-xml::Error`] for more details on the source of the error.
    ///
    /// [`quick-xml`]: ../quick_xml/index.html
    /// [`quick-xml::Error`]: ../quick_xml/enum.Error.html
    #[error("Error writing an XML tag with quick-xml")]
    WriteEventError(#[from] minidom::quick_xml::Error),
}
