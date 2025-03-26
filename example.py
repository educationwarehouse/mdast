import mdast


def main():
    # Example Markdown text
    markdown_text = """
# Welcome to Markdown Processing

This is a **bold** and *italic* text example.

## Code Block
```
def hello_world():
    print("Hello, Markdown!")
```

### List Example
1. First item
2. Second item
   - Nested item
   - Another nested item

##### Math Example
Inline math: $E = mc^2$

Block math:
$$
\int_{0}^{\infty} e^{-x^2} dx = \frac{\sqrt{\pi}}{2}
$$

"""

    # Custom parse options
    parse_options = mdast.ParseOptions(
        gfm_strikethrough=True,
        gfm_table=True,
        math_text=True,
        math_flow=True,
    )

    # Custom markdown options
    markdown_options = mdast.MarkdownOptions(
        bullet="+",
        emphasis="_",
        strong="*",
        setext=True,
        list_item_indent="tab",
    )

    print("1. Converting Markdown to AST:")
    ast = mdast.md_to_ast(markdown_text, parse_options)
    print(f"AST structure (truncated):\n{str(ast)[:500]}...\n")

    print("2. Converting AST back to Markdown:")
    regenerated_md = mdast.ast_to_md(ast, markdown_options)
    print(f"Regenerated Markdown (truncated):\n{regenerated_md[:500]}...\n")

    print("3. Converting Markdown to JSON:")
    json_ast = mdast.md_to_json(markdown_text, parse_options)
    print(f"JSON AST (truncated):\n{json_ast[:500]}...\n")

    print("4. Converting JSON back to Markdown:")
    md_from_json = mdast.json_to_md(json_ast, markdown_options)
    print(f"Markdown from JSON (truncated):\n{md_from_json[:500]}...\n")

    print("5. Demonstrating differences in output due to custom options:")
    original_list = "- First\n- Second\n   * Nested\n"
    custom_list = mdast.ast_to_md(mdast.md_to_ast(original_list), markdown_options)
    print(f"Original list:\n{original_list}")
    print(f"Custom list (with bullet '+' and 'tab' indent):\n{custom_list}")

if __name__ == "__main__":
    main()
