//! Linkify finds links such as URLs and email addresses in plain text.
//! It's smart about where a link ends, such as with trailing punctuation.
//!
//! Your reaction might be: "Do I need a library for this? Why not a regex?".
//! Let's look at a few cases:
//!
//! * In `http://example.com/.` the link should not include the trailing dot
//! * `http://example.com/,` should not include the trailing comma
//! * `(http://example.com/)` should not include the parens
//!
//! Seems simple enough. But then we also have these cases:
//!
//! * `https://en.wikipedia.org/wiki/Link_(The_Legend_of_Zelda)` should include the trailing paren
//! * `http://üñîçøðé.com/ä` should also work for Unicode (including Emoji and Punycode)
//! * `<http://example.com/>` should not include angle brackets
//!
//! This library behaves as you'd expect in the above cases and many more.
//! It uses a simple scan with linear runtime.
//!
//! In addition to URLs, it can also find emails.
//!
//! ### Usage
//!
//! Basic usage:
//!
//! ```
//! use linkify::{LinkFinder, LinkKind};
//!
//! let input = "Have you seen http://example.com?";
//! let finder = LinkFinder::new();
//! let links: Vec<_> = finder.links(input).collect();
//!
//! assert_eq!(1, links.len());
//! let link = &links[0];
//!
//! assert_eq!("http://example.com", link.as_str());
//! assert_eq!(14, link.start());
//! assert_eq!(32, link.end());
//! assert_eq!(&LinkKind::Url, link.kind());
//! ```
//!
//! Restrict the kinds of links:
//!
//! ```
//! use linkify::{LinkFinder, LinkKind};
//!
//! let input = "http://example.com and foo@example.com";
//! let mut finder = LinkFinder::new();
//! finder.kinds(&[LinkKind::Email]);
//! let links: Vec<_> = finder.links(input).collect();
//!
//! assert_eq!(1, links.len());
//! let link = &links[0];
//! assert_eq!("foo@example.com", link.as_str());
//! assert_eq!(&LinkKind::Email, link.kind());
//! ```
//!
//! Split the text into consecutive spans (mixed links and plain text).
//!
//! ```
//! use linkify::{LinkFinder, LinkKind};
//!
//! let input = "Have you seen http://example.com?";
//! let finder = LinkFinder::new();
//! let spans: Vec<_> = finder.spans(input).collect();
//!
//! assert_eq!(3, spans.len());
//!
//! assert_eq!("Have you seen ", spans[0].as_str());
//! assert_eq!(0, spans[0].start());
//! assert_eq!(14, spans[0].end());
//! assert_eq!(None, spans[0].kind());
//!
//! assert_eq!("http://example.com", spans[1].as_str());
//! assert_eq!(14, spans[1].start());
//! assert_eq!(32, spans[1].end());
//! assert_eq!(Some(&LinkKind::Url), spans[1].kind());
//!
//! assert_eq!("?", spans[2].as_str());
//! assert_eq!(32, spans[2].start());
//! assert_eq!(33, spans[2].end());
//! assert_eq!(None, spans[2].kind());
//! ```
//!
//! ### Conformance
//!
//! This crates makes an effort to respect the various standards, namely:
//!
//! * [RFC 3986] and [RFC 3987] for URLs
//! * [RFC 5321] and [RFC 6531] for emails (except IP addresses and quoting)
//!
//! At the same time, it does not guarantee that the returned links are valid.
//! If in doubt, it rather returns a link than skipping it.
//!
//! If you need to validate URLs, e.g. for checking TLDs, use another library on
//! the returned links.
//!
//! [RFC 3986]: https://tools.ietf.org/search/rfc3986
//! [RFC 3987]: https://tools.ietf.org/search/rfc3987
//! [RFC 5321]: https://tools.ietf.org/search/rfc5321
//! [RFC 6531]: https://tools.ietf.org/search/rfc6531

#![doc(html_root_url = "https://docs.rs/linkify/0.3.0")]
#![deny(warnings)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

extern crate memchr;

mod email;
mod finder;
mod url;
mod scanner;

pub use finder::Link;
pub use finder::LinkFinder;
pub use finder::LinkKind;
pub use finder::Links;
pub use finder::{Span, Spans};
