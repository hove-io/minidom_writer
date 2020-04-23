//! Writer to serialize an [`Element`] (its name, attributes and children).
//!
//! [`Element`]: ../minidom/element/struct.Element.html

use crate::Error;
use minidom::{Element, Node};
use quick_xml::{
    events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event},
    Writer,
};
use std::io::Write;
const XML_VERSION: &str = "1.0";
const ENCODING: &str = "UTF-8";

/// Writer for [`Element`].
///
/// [`Element`]: ../minidom/element/struct.Element.html
pub struct ElementWriter<W>
where
    W: Write,
{
    writer: Writer<W>,
}

// Publicly exposed methods
impl<W> ElementWriter<W>
where
    W: Write,
{
    /// Creates a [`ElementWriter`] from an instance of [`Write`].
    ///
    /// This writer writes the [`Element`] with no spaces or new lines.
    ///
    /// ```plain
    /// <?xml version="1.0" encoding="UTF-8"?><todos><todo>Write documentation</todo><todo>Release 'minidom_writer'</todo></todos>
    /// ```
    ///
    /// [`Element`]: ../minidom/element/struct.Element.html
    /// [`ElementWriter`]: struct.ElementWriter.html
    /// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    pub fn plain(write: W) -> Self {
        ElementWriter {
            writer: Writer::new(write),
        }
    }

    /// Creates a [`ElementWriter`] from an instance of [`Write`].
    ///
    /// This writer writes the [`Element`] with indentation.
    ///
    /// ```plain
    /// <?xml version="1.0" encoding="UTF-8"?>
    /// <todos>
    ///     <todo>Write documentation</todo>
    ///     <todo>Release 'minidom_writer'</todo>
    /// </todos>
    /// ```
    ///
    ///
    /// [`Element`]: ../minidom/element/struct.Element.html
    /// [`ElementWriter`]: struct.ElementWriter.html
    /// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    pub fn pretty(write: W) -> Self {
        ElementWriter {
            // 9 is ASCII code for Tabulation
            writer: Writer::new_with_indent(write, 9, 1),
        }
    }

    /// Consumes this [`ElementWriter`], returning the underlying [`Write`].
    ///
    /// [`ElementWriter`]: struct.ElementWriter.html
    /// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    pub fn into_inner(self) -> W {
        self.writer.into_inner()
    }

    /// Writes the [`Element`] into an implementation of [`Write`].
    ///
    /// [`Element`]: ../minidom/element/struct.Element.html
    /// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    pub fn write(&mut self, element: &Element) -> Result<(), Error> {
        let decl_bytes = BytesDecl::new(XML_VERSION.as_bytes(), Some(ENCODING.as_bytes()), None);
        self.writer.write_event(Event::Decl(decl_bytes))?;
        self.write_element(element)
    }
}

// Internal methods
impl<W> ElementWriter<W>
where
    W: Write,
{
    fn write_element(&mut self, element: &Element) -> Result<(), Error> {
        let name = if let Some(prefix) = element.prefix() {
            format!("{}:{}", prefix, element.name())
        } else {
            element.name().to_string()
        };
        let mut start_bytes = BytesStart::borrowed(name.as_bytes(), name.len());
        start_bytes.extend_attributes(element.attrs());
        self.writer.write_event(Event::Start(start_bytes))?;

        for node in element.nodes() {
            match node {
                Node::Element(e) => {
                    self.write_element(e)?;
                }
                Node::Text(t) => {
                    let text_bytes = BytesText::from_plain_str(t.as_str());
                    self.writer.write_event(Event::Text(text_bytes))?;
                }
                Node::Comment(_) => (),
            }
        }

        let end_bytes = BytesEnd::borrowed(name.as_bytes());
        self.writer.write_event(Event::End(end_bytes))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::io::Cursor;

    fn tag() -> Element {
        let subtag = Element::builder("ns:subtag")
            .attr("id", "my_subtag")
            .append(Node::Text(String::from("Some text")))
            .build();
        Element::builder("tag")
            .attr("id", "my_tag")
            .append(subtag)
            .build()
    }

    #[test]
    fn write_xml() {
        let tag = tag();
        let write = Cursor::new(Vec::new());
        let mut element_writer = ElementWriter::plain(write);
        element_writer.write(&tag).unwrap();
        let expected = r#"<?xml version="1.0" encoding="UTF-8"?><tag id="my_tag"><ns:subtag id="my_subtag">Some text</ns:subtag></tag>"#;
        assert_eq!(
            expected,
            String::from_utf8(element_writer.into_inner().into_inner()).unwrap()
        );
    }

    #[test]
    fn write_xml_with_indent() {
        let tag = tag();
        let write = Cursor::new(Vec::new());
        let mut element_writer = ElementWriter::pretty(write);
        element_writer.write(&tag).unwrap();
        let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
<tag id="my_tag">
	<ns:subtag id="my_subtag">Some text</ns:subtag>
</tag>"#;
        assert_eq!(
            expected,
            String::from_utf8(element_writer.into_inner().into_inner()).unwrap()
        );
    }
}
