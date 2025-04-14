// use syntect::parsing::SyntaxSet;
// use syntect::highlighting::{ThemeSet, Theme};
// use syntect::html::highlighted_html_for_string;
// use thiserror::Error;
// //use thiserror
//
// #[derive(Error, Debug)]
// pub enum HighlightError{
//     #[error("Error in highlighter")]
//     SyntaxNotFound,
//     #[error("Error in syntect: {0}")]
//     SyntectError(#[from] syntect::Error),
// }
// pub fn highlight(ss:&SyntaxSet, ts:&ThemeSet, extension:&str, data:&str) -> Result<String,HighlightError> {
//     let theme = &ts.themes["base16-ocean.dark"];
//     let out = highlighted_html_for_string(
//         data,
//         ss,
//         ss.find_syntax_by_extension(extension).ok_or(HighlightError::SyntaxNotFound)?,
//         theme,
//     );
//     Ok(out?)
// }
