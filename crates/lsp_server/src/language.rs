use std::str::FromStr;

/// Enumifies `languageId` strings from
/// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem.
/// (but on the ones we might care about).
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum LanguageId {
    Ruby,
}

impl FromStr for LanguageId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ruby" => Ok(Self::Ruby),
            _ => Err(()),
        }
    }
}
