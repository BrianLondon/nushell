use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Category, Example, PipelineData, ShellError, Signature, Span, Spanned, SyntaxShape, Type,
    Value,
};
use regex::Regex;
#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "split row"
    }

    fn signature(&self) -> Signature {
        Signature::build("split row")
            .input_output_types(vec![(Type::String, Type::List(Box::new(Type::String)))])
            .vectorizes_over_list(true)
            .required(
                "separator",
                SyntaxShape::String,
                "a character or regex that denotes what separates rows",
            )
            .named(
                "number",
                SyntaxShape::Int,
                "Split into maximum number of items",
                Some('n'),
            )
            .switch("regex", "use regex syntax for separator", Some('r'))
            .category(Category::Strings)
    }

    fn usage(&self) -> &str {
        "Split a string into multiple rows using a separator."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["separate", "divide", "regex"]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        split_row(engine_state, stack, call, input)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Split a string into rows of char",
                example: "'abc' | split row ''",
                result: Some(Value::List {
                    vals: vec![
                        Value::test_string(""),
                        Value::test_string("a"),
                        Value::test_string("b"),
                        Value::test_string("c"),
                        Value::test_string(""),
                    ],
                    span: Span::test_data(),
                }),
            },
            Example {
                description: "Split a string into rows by the specified separator",
                example: "'a--b--c' | split row '--'",
                result: Some(Value::List {
                    vals: vec![
                        Value::test_string("a"),
                        Value::test_string("b"),
                        Value::test_string("c"),
                    ],
                    span: Span::test_data(),
                }),
            },
            Example {
                description: "Split a string by '-'",
                example: "'-a-b-c-' | split row '-'",
                result: Some(Value::List {
                    vals: vec![
                        Value::test_string(""),
                        Value::test_string("a"),
                        Value::test_string("b"),
                        Value::test_string("c"),
                        Value::test_string(""),
                    ],
                    span: Span::test_data(),
                }),
            },
            Example {
                description: "Split a string by regex",
                example: r"'a   b       c' | split row -r '\s+'",
                result: Some(Value::List {
                    vals: vec![
                        Value::test_string("a"),
                        Value::test_string("b"),
                        Value::test_string("c"),
                    ],
                    span: Span::test_data(),
                }),
            },
        ]
    }
}

fn split_row(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    input: PipelineData,
) -> Result<PipelineData, ShellError> {
    let name_span = call.head;
    let separator: Spanned<String> = call.req(engine_state, stack, 0)?;
    let regex = if call.has_flag("regex") {
        Regex::new(&separator.item)
    } else {
        let escaped = regex::escape(&separator.item);
        Regex::new(&escaped)
    }
    .map_err(|err| {
        ShellError::GenericError(
            "Error with regular expression".into(),
            err.to_string(),
            Some(separator.span),
            None,
            Vec::new(),
        )
    })?;
    let max_split: Option<usize> = call.get_flag(engine_state, stack, "number")?;
    input.flat_map(
        move |x| split_row_helper(&x, &regex, max_split, name_span),
        engine_state.ctrlc.clone(),
    )
}

fn split_row_helper(v: &Value, regex: &Regex, max_split: Option<usize>, name: Span) -> Vec<Value> {
    match v.span() {
        Ok(v_span) => {
            if let Ok(s) = v.as_string() {
                match max_split {
                    Some(max_split) => regex
                        .splitn(&s, max_split)
                        .map(|x: &str| Value::string(x, v_span))
                        .collect(),
                    None => regex
                        .split(&s)
                        .map(|x: &str| Value::string(x, v_span))
                        .collect(),
                }
            } else {
                vec![Value::Error {
                    error: Box::new(ShellError::PipelineMismatch {
                        exp_input_type: "string".into(),
                        dst_span: name,
                        src_span: v_span,
                    }),
                }]
            }
        }
        Err(error) => vec![Value::Error {
            error: Box::new(error),
        }],
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(SubCommand {})
    }
}
