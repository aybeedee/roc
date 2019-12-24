use crate::fmt::spaces::fmt_spaces;
use crate::parse::ast::{Base, Pattern};
use bumpalo::collections::String;

pub fn fmt_pattern<'a>(
    buf: &mut String<'a>,
    pattern: &'a Pattern<'a>,
    indent: u16,
    apply_needs_parens: bool,
) {
    use self::Pattern::*;

    match pattern {
        Identifier(string) => buf.push_str(string),
        GlobalTag(name) | PrivateTag(name) => {
            buf.push_str(name);
        }
        Apply(loc_pattern, loc_arg_patterns) => {
            if apply_needs_parens {
                buf.push('(');
            }

            fmt_pattern(buf, &loc_pattern.value, indent, true);

            for loc_arg in loc_arg_patterns.iter() {
                buf.push(' ');
                fmt_pattern(buf, &loc_arg.value, indent, true);
            }

            if apply_needs_parens {
                buf.push(')');
            }
        }
        RecordDestructure(loc_patterns) => {
            buf.push_str("{ ");

            let mut is_first = true;

            for loc_pattern in loc_patterns {
                if is_first {
                    is_first = false;
                } else {
                    buf.push_str(", ");
                }

                fmt_pattern(buf, &loc_pattern.value, indent, true);
            }

            buf.push_str(" }");
        }

        RecordField(name, loc_pattern) => {
            buf.push_str(name);
            buf.push_str(": ");
            fmt_pattern(buf, &loc_pattern.value, indent, true);
        }

        IntLiteral(string) => buf.push_str(string),
        NonBase10Literal {
            base,
            string,
            is_negative,
        } => {
            if *is_negative {
                buf.push('-');
            }

            buf.push('0');

            buf.push(match base {
                Base::Hex => 'x',
                Base::Octal => 'o',
                Base::Binary => 'b',
            });

            buf.push_str(string);
        }
        FloatLiteral(string) => buf.push_str(string),
        StrLiteral(string) => buf.push_str(string),
        BlockStrLiteral(lines) => {
            for line in *lines {
                buf.push_str(line)
            }
        }
        Underscore => buf.push('_'),

        // Space
        SpaceBefore(sub_pattern, spaces) => {
            fmt_spaces(buf, spaces.iter(), indent);
            fmt_pattern(buf, sub_pattern, indent, apply_needs_parens);
        }
        SpaceAfter(sub_pattern, spaces) => {
            fmt_pattern(buf, sub_pattern, indent, apply_needs_parens);
            fmt_spaces(buf, spaces.iter(), indent);
        }

        Nested(sub_pattern) => {
            fmt_pattern(buf, sub_pattern, indent, apply_needs_parens);
        }

        // Malformed
        Malformed(string) => buf.push_str(string),
        QualifiedIdentifier(maybe_qualified) => {
            for part in maybe_qualified.module_parts.iter() {
                buf.push_str(part);
                buf.push('.');
            }

            buf.push_str(maybe_qualified.value);
        }
    }
}
