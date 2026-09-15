use crate::prelude::*;
use crate::utils::scss_separator_comments::FormatScssSeparatorComments;
use biome_css_syntax::{
    CssBogusPropertyValue, CssParameterList, ScssExpression, ScssExpressionFields,
    is_in_scss_include_arguments, single_expression_item,
};
use biome_formatter::{FormatResult, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssExpression;
impl FormatNodeRule<ScssExpression> for FormatScssExpression {
    fn fmt_node(&self, node: &ScssExpression, f: &mut CssFormatter) -> FormatResult<()> {
        if is_function_argument_with_leading_comments(node, f) {
            // Keep the comment and function aligned. Only breaks inside the
            // function inherit the additional argument-body indent.
            write!(
                f,
                [
                    format_leading_comments(node.syntax()).with_following_content(|f| {
                        let formatted = format_with(|f| self.fmt_fields(node, f));
                        write!(f, [indent(&formatted)])
                    })
                ]
            )
        } else {
            self.fmt_node_with_scss_separator_comments(node, f)
        }
    }

    fn fmt_fields(&self, node: &ScssExpression, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssExpressionFields { items } = node.as_fields();

        let has_enclosing_expression = node
            .syntax()
            .ancestors()
            .skip(1)
            .any(|ancestor| ScssExpression::can_cast(ancestor.kind()));

        if !has_enclosing_expression
            && items
                .syntax()
                .descendants()
                .any(|descendant| CssBogusPropertyValue::can_cast(descendant.kind()))
        {
            // Recovery can split opaque syntax like `progid:...(...,...)` into list items.
            return write!(f, [format_bogus_node(items.syntax())]);
        }

        write!(f, [items.format()])
    }

    fn fmt_leading_comments(
        &self,
        node: &ScssExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        if is_function_argument_with_leading_comments(node, f) {
            Ok(())
        } else {
            self.fmt_leading_scss_separator_comments(node, f)
        }
    }
}

fn is_function_argument_with_leading_comments(node: &ScssExpression, f: &CssFormatter) -> bool {
    node.parent::<CssParameterList>().is_some()
        && f.comments().has_leading_comments(node.syntax())
        && single_expression_item(node).is_some_and(|item| {
            item.as_any_css_value()
                .is_some_and(|value| value.as_any_css_function().is_some())
        })
        && !is_in_scss_include_arguments(node.syntax())
}
