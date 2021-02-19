[![GitHub Action Status][github_actions_badge]][github_actions]
[![Latest Version]][crates.io]

[github_actions_badge]: https://img.shields.io/github/workflow/status/CanalTP/minidom_writer/Continuous%20Integration
[github_actions]: https://github.com/CanalTP/minidom_writer/actions
[Latest Version]: https://img.shields.io/crates/v/minidom_writer.svg    
[crates.io]: https://crates.io/crates/minidom_writer                    

⚠ This project is archived and last `minidom` version supported is `0.13.0` ⚠
`minidom` now provides substantial functionality to write an XML file, see the following API:

- [`Element::to_writer()`][to_writer]
- [`Element::to_writer_decl()`][to_writer_decl]
- [`Element::write_to()`][write_to]
- [`Element::write_to_decl()`][write_to_decl]
- [`Element::write_to_inner()`][write_to_inner]

[to_writer]: https://docs.rs/minidom/0.13.0/minidom/element/struct.Element.html#method.to_writer
[to_writer_decl]: https://docs.rs/minidom/0.13.0/minidom/element/struct.Element.html#method.to_writer_decl
[write_to]: https://docs.rs/minidom/0.13.0/minidom/element/struct.Element.html#method.write_to
[write_to_decl]: https://docs.rs/minidom/0.13.0/minidom/element/struct.Element.html#method.write_to_decl
[write_to_inner]: https://docs.rs/minidom/0.13.0/minidom/element/struct.Element.html#method.write_to_inner

# minidom_writer
Helper to write 'minidom::Element' into an XML. See [documentation](https://docs.rs/minidom_writer).
