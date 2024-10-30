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
            assertion_kind,
            colon_token,
            l_curly_token,
            assertions,
            r_curly_token,
        } = node.as_fields();

        let should_insert_space_around_brackets = f.options().bracket_spacing().value();

        write![
            f,
            [
                assertion_kind.format(),
                colon_token.format(),
                space(),
                l_curly_token.format(),
                space(),
                group(&soft_block_indent_with_maybe_space(
                    &assertions.format(),
                    should_insert_space_around_brackets
                )),
                r_curly_token.format()
            ]
        ]
    }
}
