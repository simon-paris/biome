use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::TsImportTypeAssertionContainer;
use biome_js_syntax::TsImportTypeAssertionContainerFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsImportTypeAssertionContainer;

impl FormatNodeRule<TsImportTypeAssertionContainer> for FormatTsImportTypeAssertionContainer {
    fn fmt_fields(
        &self,
        node: &TsImportTypeAssertionContainer,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsImportTypeAssertionContainerFields {
            l_curly_token,
            assertion_kind_token,
            assert_token: _assert_token,
            colon_token,
            assert_clause,
            r_curly_token,
        } = node.as_fields();

        let should_insert_space_around_brackets = f.options().bracket_spacing().value();

        write![
            f,
            [
                l_curly_token.format(),
                space(),
                assertion_kind_token.format(),
                colon_token.format(),
                space(),
                group(&soft_block_indent_with_maybe_space(
                    &assert_clause.format(),
                    should_insert_space_around_brackets
                )),
                space(),
                r_curly_token.format()
            ]
        ]
    }
}
