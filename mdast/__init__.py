"""
Python typing/documentation layer.
Actual implementation exists in Rust.
"""

import dataclasses
import typing as t

# Rust code:
from . import mdast as _mdast  # type: ignore

__all__ = [
    "ParseOptions",
    "MarkdownOptions",
    "ast_to_md",
    "json_to_md",
    "md_to_ast",
    "md_to_json",
]


@dataclasses.dataclass
class ParseOptions:
    """
    Configuration options for parsing Markdown.

    This class represents the various options available for parsing Markdown text,
    including support for different Markdown extensions and syntax features.

    See Also:
        https://docs.rs/markdown/1.0.0-alpha.23/markdown/struct.ParseOptions.html
        https://docs.rs/markdown/1.0.0-alpha.23/markdown/struct.Constructs.html
    """

    gfm_strikethrough_single_tilde: bool = True
    math_text_single_dollar: bool = True
    attention: bool = True
    autolink: bool = True
    block_quote: bool = True
    character_escape: bool = True
    character_reference: bool = True
    code_indented: bool = True
    code_fenced: bool = True
    code_text: bool = True
    definition: bool = True
    frontmatter: bool = False
    gfm_autolink_literal: bool = False
    gfm_label_start_footnote: bool = False
    gfm_footnote_definition: bool = False
    gfm_strikethrough: bool = False
    gfm_table: bool = False
    gfm_task_list_item: bool = False
    hard_break_escape: bool = True
    hard_break_trailing: bool = True
    heading_atx: bool = True
    heading_setext: bool = True
    html_flow: bool = True
    html_text: bool = True
    label_start_image: bool = True
    label_start_link: bool = True
    label_end: bool = True
    list_item: bool = True
    math_flow: bool = False
    math_text: bool = False
    mdx_esm: bool = False
    mdx_expression_flow: bool = False
    mdx_expression_text: bool = False
    mdx_jsx_flow: bool = False
    mdx_jsx_text: bool = False
    thematic_break: bool = True


@dataclasses.dataclass
class MarkdownOptions:
    """
    Configuration options for converting AST to Markdown.

    This class represents the various options available for customizing the
    Markdown output when converting from an Abstract Syntax Tree (AST) representation.

    See Also:
        https://docs.rs/mdast_util_to_markdown/0.0.1/mdast_util_to_markdown/struct.Options.html
    """

    bullet: str = "*"
    bullet_ordered: str = "."
    bullet_other: str = "-"
    close_atx: bool = False
    emphasis: str = "*"
    fence: str = "`"
    fences: bool = True
    increment_list_marker: bool = True
    list_item_indent: t.Literal["mixed", "one", "tab"] = "one"
    quote: str = '"'
    resource_link: bool = False
    rule: str = "*"
    rule_repetition: int = 3
    rule_spaces: bool = False
    setext: bool = False
    single_dollar_text_math: bool = True
    strong: str = "*"
    tight_definitions: bool = False


def asdict(config: dict | None | t.Any) -> dict | None:
    """
    Turn a 'config' class into a dict so Rust can use it.

    This function converts a configuration object or dictionary into a format
    that can be used by the Rust backend.

    Args:
        config (dict | None | t.Any): The configuration object or dictionary to convert.

    Returns:
        dict | None: The converted dictionary, or None if the input is falsy.

    Meant for internal use.
    """
    if not config:
        return None

    if isinstance(config, dict):
        return config

    return dataclasses.asdict(config)


def md_to_json(md: str, config: t.Optional[ParseOptions] = None) -> str:
    """
    Convert Markdown text to a JSON representation of the AST.

    Args:
        md (str): The Markdown text to convert.
        config (Optional[ParseOptions]): Configuration options for parsing.

    Returns:
        str: A JSON string representing the Markdown AST.
    """
    return _mdast.md_to_json(md, asdict(config))


def md_to_ast(md: str, config: t.Optional[ParseOptions] = None) -> dict:
    """
    Convert Markdown text to an Abstract Syntax Tree (AST) representation.

    Args:
        md (str): The Markdown text to convert.
        config (Optional[ParseOptions]): Configuration options for parsing.

    Returns:
        dict: A dictionary representing the Markdown AST.
    """
    return _mdast.md_to_ast(md, asdict(config))


def json_to_md(json: str, config: t.Optional[MarkdownOptions] = None) -> str:
    """
    Convert a JSON representation of an AST back to Markdown text.

    Args:
        json (str): The JSON string representing the Markdown AST.
        config (Optional[MarkdownOptions]): Configuration options for Markdown generation.

    Returns:
        str: The generated Markdown text.
    """
    return _mdast.json_to_md(json, asdict(config))


def ast_to_md(ast: dict, config: t.Optional[MarkdownOptions] = None) -> str:
    """
    Convert an Abstract Syntax Tree (AST) representation to Markdown text.

    Args:
        ast (dict): The dictionary representing the Markdown AST.
        config (Optional[MarkdownOptions]): Configuration options for Markdown generation.

    Returns:
        str: The generated Markdown text.
    """
    return _mdast.ast_to_md(ast, asdict(config))
