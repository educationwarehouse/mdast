use markdown::mdast::Node;
use markdown::{
    to_html_with_options, to_mdast, CompileOptions, Constructs, LineEnding, ParseOptions,
};
use mdast_util_to_markdown::{to_markdown_with_options, IndentOptions};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::{pyfunction, pymodule, PyModule, PyModuleMethods, PyResult};
use pyo3::types::PyDict;
use pyo3::{wrap_pyfunction, Bound, PyAny, Python};
use serde::{Deserialize, Serialize};
use serde_pyobject::{from_pyobject, to_pyobject};

use mdast_util_to_markdown::Options as MarkdownOptions;

trait FromPy {
    fn from_py(config: Option<Bound<PyAny>>) -> Self;
}

impl FromPy for ParseOptions {
    fn from_py(config: Option<Bound<PyAny>>) -> Self {
        let mut parse_options = ParseOptions::default();

        if let Some(config) = config {
            match from_pyobject::<PyParseOptions, _>(config) {
                Ok(config) => parse_options = config.into(),
                Err(e) => {
                    eprintln!("Error parsing parse config: {e}")
                }
            }
        }

        parse_options
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
/// https://docs.rs/markdown/1.0.0-alpha.23/markdown/struct.ParseOptions.html
/// https://docs.rs/markdown/1.0.0-alpha.23/markdown/struct.Constructs.html
struct PyParseOptions {
    // todo: support `variant` (commonmark/gfm/mdx)
    gfm_strikethrough_single_tilde: bool,
    math_text_single_dollar: bool,
    attention: bool,
    autolink: bool,
    block_quote: bool,
    character_escape: bool,
    character_reference: bool,
    code_indented: bool,
    code_fenced: bool,
    code_text: bool,
    definition: bool,
    frontmatter: bool,
    gfm_autolink_literal: bool,
    gfm_label_start_footnote: bool,
    gfm_footnote_definition: bool,
    gfm_strikethrough: bool,
    gfm_table: bool,
    gfm_task_list_item: bool,
    hard_break_escape: bool,
    hard_break_trailing: bool,
    heading_atx: bool,
    heading_setext: bool,
    html_flow: bool,
    html_text: bool,
    label_start_image: bool,
    label_start_link: bool,
    label_end: bool,
    list_item: bool,
    math_flow: bool,
    math_text: bool,
    mdx_esm: bool,
    mdx_expression_flow: bool,
    mdx_expression_text: bool,
    mdx_jsx_flow: bool,
    mdx_jsx_text: bool,
    thematic_break: bool,
}

impl From<PyParseOptions> for ParseOptions {
    fn from(val: PyParseOptions) -> Self {
        let constructs = Constructs {
            attention: val.attention,
            autolink: val.autolink,
            block_quote: val.block_quote,
            character_escape: val.character_escape,
            character_reference: val.character_reference,
            code_indented: val.code_indented,
            code_fenced: val.code_fenced,
            code_text: val.code_text,
            definition: val.definition,
            frontmatter: val.frontmatter,
            gfm_autolink_literal: val.gfm_autolink_literal,
            gfm_footnote_definition: val.gfm_footnote_definition,
            gfm_label_start_footnote: val.gfm_label_start_footnote,
            gfm_strikethrough: val.gfm_strikethrough,
            gfm_table: val.gfm_table,
            gfm_task_list_item: val.gfm_task_list_item,
            hard_break_escape: val.hard_break_escape,
            hard_break_trailing: val.hard_break_trailing,
            heading_atx: val.heading_atx,
            heading_setext: val.heading_setext,
            html_flow: val.html_flow,
            html_text: val.html_text,
            label_start_image: val.label_start_image,
            label_start_link: val.label_start_link,
            label_end: val.label_end,
            list_item: val.list_item,
            math_flow: val.math_flow,
            math_text: val.math_text,
            mdx_esm: val.mdx_esm,
            mdx_expression_flow: val.mdx_expression_flow,
            mdx_expression_text: val.mdx_expression_text,
            mdx_jsx_flow: val.mdx_jsx_flow,
            mdx_jsx_text: val.mdx_jsx_text,
            thematic_break: val.thematic_break,
        };

        ParseOptions {
            constructs,
            gfm_strikethrough_single_tilde: val.gfm_strikethrough_single_tilde,
            math_text_single_dollar: val.math_text_single_dollar,
            mdx_expression_parse: None,
            mdx_esm_parse: None,
        }
    }
}

fn to_mdast_py(md: &str, config: Option<Bound<PyAny>>) -> PyResult<Node> {
    let parse_options = ParseOptions::from_py(config);
    let tree = to_mdast(md, &parse_options)
        .map_err(|e| PyRuntimeError::new_err(format!("Failed to parse Markdown: {e}")))?;

    Ok(tree)
}

#[pyfunction]
#[pyo3(signature = (md, config=None))]
/// Convert markdown to an mdast json (string) representation.
fn md_to_json(_py: Python<'_>, md: &str, config: Option<Bound<PyAny>>) -> PyResult<String> {
    let tree = to_mdast_py(md, config)?;

    let json = serde_json::to_string(&tree)
        .map_err(|e| PyRuntimeError::new_err(format!("Failed to convert to JSON: {e}")))?;
    Ok(json)
}

#[pyfunction]
#[pyo3(signature = (md, config=None))]
/// Convert markdown to an mdast ast (dict) representation.
fn md_to_ast<'py>(
    py: Python<'py>,
    md: &'py str,
    config: Option<Bound<PyAny>>,
) -> PyResult<Bound<'py, PyAny>> {
    let tree = to_mdast_py(md, config)?;

    to_pyobject(py, &tree).map_err(|_| PyRuntimeError::new_err("Failed to create AST dict"))
}

#[derive(Deserialize, Serialize, Debug, Default)]
/// https://docs.rs/mdast_util_to_markdown/0.0.1/mdast_util_to_markdown/struct.Options.html
struct PyMarkdownOptions {
    bullet: char,
    bullet_ordered: char,
    bullet_other: char,
    close_atx: bool,
    emphasis: char,
    fence: char,
    fences: bool,
    increment_list_marker: bool,
    list_item_indent: String,
    quote: char,
    resource_link: bool,
    rule: char,
    rule_repetition: u32,
    rule_spaces: bool,
    setext: bool,
    single_dollar_text_math: bool,
    strong: char,
    tight_definitions: bool,
}

impl From<PyMarkdownOptions> for MarkdownOptions {
    fn from(val: PyMarkdownOptions) -> Self {
        let list_item_indent: IndentOptions = match val.list_item_indent.to_lowercase().as_ref() {
            "mixed" => IndentOptions::Mixed,
            "one" => IndentOptions::One,
            "tab" => IndentOptions::Tab,
            _ => unimplemented!("Unsupported `list_item_indent`"),
        };

        MarkdownOptions {
            bullet: val.bullet,
            bullet_ordered: val.bullet_ordered,
            bullet_other: val.bullet_other,
            close_atx: val.close_atx,
            emphasis: val.emphasis,
            fence: val.fence,
            fences: val.fences,
            increment_list_marker: val.increment_list_marker,
            list_item_indent,
            quote: val.quote,
            resource_link: val.resource_link,
            rule: val.rule,
            rule_repetition: val.rule_repetition,
            rule_spaces: val.rule_spaces,
            setext: val.setext,
            single_dollar_text_math: val.single_dollar_text_math,
            strong: val.strong,
            tight_definitions: val.tight_definitions,
        }
    }
}

impl FromPy for MarkdownOptions {
    fn from_py(config: Option<Bound<PyAny>>) -> Self {
        let mut md_options = MarkdownOptions::default();

        if let Some(config) = config {
            match from_pyobject::<PyMarkdownOptions, _>(config) {
                Ok(config) => md_options = config.into(),
                Err(e) => {
                    eprintln!("Error parsing Markdown config: {e}")
                }
            }
        }

        md_options
    }
}

fn to_markdown_py(tree: &Node, config: Option<Bound<PyAny>>) -> PyResult<String> {
    let md_options = MarkdownOptions::from_py(config);

    let md = to_markdown_with_options(tree, &md_options).map_err(|e| {
        PyRuntimeError::new_err(format!("Failed to convert node back to markdown: {e}"))
    })?;

    Ok(md)
}

#[pyfunction]
#[pyo3(signature = (json, config=None))]
/// Convert mdast json back to markdown.
fn json_to_md(_py: Python<'_>, json: &str, config: Option<Bound<PyAny>>) -> PyResult<String> {
    let tree: Node = serde_json::from_str(json)
        .map_err(|e| PyRuntimeError::new_err(format!("Failed to parse json back to node: {e}")))?;

    to_markdown_py(&tree, config)
}

#[pyfunction]
#[pyo3(signature = (dict, config=None))]
fn ast_to_md(
    _py: Python<'_>,
    dict: Bound<PyDict>,
    config: Option<Bound<PyAny>>,
) -> PyResult<String> {
    let tree: Node = from_pyobject(dict)
        .map_err(|e| PyRuntimeError::new_err(format!("Failed to parse ast back to node: {e}")))?;

    to_markdown_py(&tree, config)
}

#[derive(Deserialize, Serialize, Debug, Default)]
/// https://docs.rs/markdown/1.0.0-alpha.23/markdown/struct.CompileOptions.html
struct PyCompileOptions {
    allow_dangerous_html: bool,
    allow_dangerous_protocol: bool,
    default_line_ending: String,
    gfm_footnote_label: Option<String>,
    gfm_footnote_label_tag_name: Option<String>,
    gfm_footnote_label_attributes: Option<String>,
    gfm_footnote_back_label: Option<String>,
    gfm_footnote_clobber_prefix: Option<String>,
    gfm_task_list_item_checkable: bool,
    gfm_tagfilter: bool,
}

impl FromPy for CompileOptions {
    fn from_py(config: Option<Bound<PyAny>>) -> Self {
        let mut compile_options = CompileOptions::default();

        if let Some(config) = config {
            match from_pyobject::<PyCompileOptions, _>(config) {
                Ok(config) => compile_options = config.into(),
                Err(e) => {
                    eprintln!("Error parsing compile config: {e}")
                }
            }
        }

        compile_options
    }
}

impl From<PyCompileOptions> for CompileOptions {
    fn from(value: PyCompileOptions) -> Self {
        let default_line_ending: LineEnding = match value.default_line_ending.as_ref() {
            "\n" => LineEnding::LineFeed,
            "\r\n" => LineEnding::CarriageReturnLineFeed,
            "\r" => LineEnding::CarriageReturn,
            _ => unimplemented!("Unsupported `default_line_ending`"),
        };

        Self {
            allow_dangerous_html: value.allow_dangerous_html,
            allow_dangerous_protocol: value.allow_dangerous_protocol,
            default_line_ending,
            gfm_footnote_label: value.gfm_footnote_label,
            gfm_footnote_label_tag_name: value.gfm_footnote_label_tag_name,
            gfm_footnote_label_attributes: value.gfm_footnote_label_attributes,
            gfm_footnote_back_label: value.gfm_footnote_back_label,
            gfm_footnote_clobber_prefix: value.gfm_footnote_clobber_prefix,
            gfm_task_list_item_checkable: value.gfm_task_list_item_checkable,
            gfm_tagfilter: value.gfm_tagfilter,
        }
    }
}

#[pyfunction]
#[pyo3(signature = (md, parse_config=None, compile_config=None))]
fn md_to_html(
    _py: Python<'_>,
    md: &str,
    parse_config: Option<Bound<PyAny>>,
    compile_config: Option<Bound<PyAny>>,
) -> PyResult<String> {
    let parse_options = ParseOptions::from_py(parse_config);
    let compile_options = CompileOptions::from_py(compile_config);

    let options = markdown::Options {
        parse: parse_options,
        compile: compile_options,
    };

    to_html_with_options(md, &options)
        .map_err(|e| PyRuntimeError::new_err(format!("Failed to convert markdown to html: {e}")))
}

#[pyfunction]
#[pyo3(signature = (json, md_config=None, parse_config=None, compile_config=None))]
fn json_to_html(
    py: Python<'_>,
    json: &str,
    md_config: Option<Bound<PyAny>>,
    parse_config: Option<Bound<PyAny>>,
    compile_config: Option<Bound<PyAny>>,
) -> PyResult<String> {
    // mdast current only supports md->html so convert back to md first:
    let md = json_to_md(py, json, md_config)?;
    md_to_html(py, &md, parse_config, compile_config)
}

#[pyfunction]
#[pyo3(signature = (dict, md_config=None, parse_config=None, compile_config=None))]
fn ast_to_html(
    py: Python<'_>,
    dict: Bound<PyDict>,
    md_config: Option<Bound<PyAny>>,
    parse_config: Option<Bound<PyAny>>,
    compile_config: Option<Bound<PyAny>>,
) -> PyResult<String> {
    // mdast current only supports md->html so convert back to md first:
    let md = ast_to_md(py, dict, md_config)?;
    md_to_html(py, &md, parse_config, compile_config)
}

#[pymodule]
fn mdast(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(md_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(json_to_md, m)?)?;
    m.add_function(wrap_pyfunction!(md_to_ast, m)?)?;
    m.add_function(wrap_pyfunction!(ast_to_md, m)?)?;
    m.add_function(wrap_pyfunction!(md_to_html, m)?)?;
    m.add_function(wrap_pyfunction!(json_to_html, m)?)?;
    m.add_function(wrap_pyfunction!(ast_to_html, m)?)?;
    Ok(())
}
