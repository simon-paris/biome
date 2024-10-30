use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::TsImportType;
use biome_js_syntax::TsImportTypeFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsImportType;

impl FormatNodeRule<TsImportType> for FormatTsImportType {
    fn fmt_fields(&self, node: &TsImportType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsImportTypeFields {
            typeof_token,
            import_token,
            l_paren_token,
            argument,
            comma_token,
            assertions,
            r_paren_token,
            qualifier_clause,
            type_arguments,
        } = node.as_fields();

        if let Some(typeof_token) = typeof_token {
            write!(f, [typeof_token.format(), space()])?;
        }

        write![
            f,
            [
                import_token.format(),
                l_paren_token.format(),
                argument.format(),
            ]
        ]?;

        if let Some(comma_token) = comma_token {
            write![f, [comma_token.format(), assertions.format(),]]?;
        }

        write![
            f,
            [
                r_paren_token.format(),
                qualifier_clause.format(),
                type_arguments.format(),
            ]
        ]
    }
}
