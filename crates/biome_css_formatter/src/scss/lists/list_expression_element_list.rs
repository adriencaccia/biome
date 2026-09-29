use crate::prelude::*;
use crate::utils::scss_list_layout::has_singleton_list_separator;
use biome_css_syntax::ScssListExpressionElementList;
use biome_formatter::separated::TrailingSeparator;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssListExpressionElementList;

impl FormatRule<ScssListExpressionElementList> for FormatScssListExpressionElementList {
    type Context = CssFormatContext;

    fn fmt(&self, node: &ScssListExpressionElementList, f: &mut CssFormatter) -> FormatResult<()> {
        // A singleton comma distinguishes a list from its only value.
        let trailing_separator = if has_singleton_list_separator(node) {
            TrailingSeparator::Mandatory
        } else {
            TrailingSeparator::Omit
        };
        let separator = soft_line_break_or_space();
        let mut joiner = f.join_with(&separator);
        let separated = node
            .format_separated(",")
            .with_trailing_separator(trailing_separator);

        for formatted in separated {
            joiner.entry(&formatted);
        }
        joiner.finish()
    }
}
