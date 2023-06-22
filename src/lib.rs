//! # `humanise` - humanise information.
//!
//! `humanise` is a library that provides functions to convert raw data to not just human-readable strings, but humanised strings.

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]

use std::fmt::Display;

#[cfg(feature = "durations")]
#[cfg_attr(docsrs, doc(cfg(feature = "durations")))]
pub mod durations;
#[cfg(feature = "durations")]
#[cfg_attr(docsrs, doc(cfg(feature = "durations")))]
pub use durations::*;

/// Humanise a list.
///
/// # Arguments
///
/// * `list`: The list to humanise.
///
/// # Return value
///
/// If there are no elements in `list`, an empty string is returned.
/// If there is one element in `list`, that element is formatted and returned.
/// If there are two elements in `list`, those elements are formatted and concatenated with an and in the middle.
/// If there are more elements in `list`, the elements are listed with a serial comma (i.e. a comma after the penultimate term).
///
/// # Examples
///
/// ```
/// use humanise::humanise_list;
///
/// assert_eq!(humanise_list(&["apples"]), "apples");
/// assert_eq!(humanise_list(&["apples", "bananas"]), "apples and bananas");
/// assert_eq!(humanise_list(&["apples", "bananas", "strawberries"]), "apples, bananas, and strawberries");
/// ```
pub fn humanise_list<T>(list: &[T]) -> String
where
    T: Display,
{
    match list.len() {
        0 => "".to_string(),
        1 => list.first().unwrap().to_string(),
        2 => format!("{} and {}", list.first().unwrap(), list.last().unwrap()),
        _ => {
            let mut str = String::new();
            for (i, value) in list.iter().enumerate() {
                let prefix = match i {
                    0 => "",
                    len if len == list.len() - 1 => ", and ",
                    _ => ", ",
                };
                str = format!("{}{}{}", str, prefix, value);
            }
            str
        }
    }
}
/// Adds a plural suffix if there is supposed to be one.
///
/// # Arguments
///
/// * `count`: The number of items. The suffix is applied if this is not 1.
/// * `word`: The word to apply the suffix to.
/// * `opposite`: Determines the suffix to use. `false` makes it so that plural gets the `s` suffix - `true` would have singular get the prefix instead.
///               This is included so that this function can be used with both nouns and verbs.
///
/// # Return value
///
/// `word`, optionally suffixed with `s` depending on the plurality (determined by `count`) and `opposite`.
///
/// # Examples
///
/// ```
/// use humanise::plural_suffix;
///
/// // Most people set the `opposite` parameter to false.
/// // - One apple, so no suffix is applied.
/// assert_eq!(plural_suffix(1, "apple", false), "apple");
/// // - Five apples - suffix appears!
/// assert_eq!(plural_suffix(5, "apple", false), "apples");
///
/// // Using opposite, we can use this function for verbs (`count` is used to determine the plurality of the subject in this case).
/// // - One machine makes something.
/// assert_eq!(plural_suffix(1, "make", true), "makes");
/// // - Five machines make something.
/// assert_eq!(plural_suffix(5, "make", true), "make");
/// ```
#[inline]
pub fn plural_suffix(count: u128, word: impl AsRef<str>, opposite: bool) -> String {
    let count = count.into();
    let suffix = match count {
        1 => {
            if opposite {
                "s"
            } else {
                ""
            }
        }
        _ => {
            if opposite {
                ""
            } else {
                "s"
            }
        }
    };
    format!("{}{}", word.as_ref(), suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plural_suffix_opposite_false() {
        assert_eq!(plural_suffix(0, "name", false), "names");
        assert_eq!(plural_suffix(1, "name", false), "name");
        assert_eq!(plural_suffix(2, "name", false), "names");
    }
    #[test]
    fn plural_suffix_opposite_true() {
        assert_eq!(plural_suffix(0, "make", true), "make");
        assert_eq!(plural_suffix(1, "make", true), "makes");
        assert_eq!(plural_suffix(2, "make", true), "make");
    }
}
