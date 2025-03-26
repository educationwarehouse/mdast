# mdast.py

Simple Python bindings for the [mdast](https://github.com/syntax-tree/mdast) functionality of [wooorm/markdown-rs](https://github.com/wooorm/markdown-rs/)

## Installation

```bash
pip install mdast
```

If you're on x86-64/AMD64 or arm64/aarch64, you can install this package without having Rust on your system.
For other platforms, the Rust toolchain is required to build the binary dependencies.

## Usage

### Converting from Markdown to mdast's AST format

```python
import mdast

mdast.md_to_ast("# title")
# -> {'type': 'root', 'children': [{'type': 'heading', 'children': [{'type': 'text', 'value': 'title'}], 'depth': 1}]}
```

### Converting from Markdown to mdast's JSON format

```python
import mdast

mdast.md_to_json("# title")
# -> '{"type": "root", "children": [{"type": "heading", "children": [{"type": "text", "value": "title"}], "depth": 1}]}'
```

### Converting from mdast AST to Markdown

```python
import mdast

ast = {'type': 'root', 'children': [{'type': 'heading', 'children': [{'type': 'text', 'value': 'title'}], 'depth': 1}]}
mdast.ast_to_md(ast)
# -> '# title\n'
```

### Converting from mdast JSON to Markdown

```python
import mdast

json_str = '{"type": "root", "children": [{"type": "heading", "children": [{"type": "text", "value": "title"}], "depth": 1}]}'
mdast.json_to_md(json_str)
# -> '# title\n'
```

### Converting from Markdown to HTML

```python
import mdast

mdast.md_to_html("# title")
# -> '<h1>title</h1>'
```

### Converting from mdast AST to HTML

```python
import mdast

ast = {'type': 'root', 'children': [{'type': 'heading', 'children': [{'type': 'text', 'value': 'title'}], 'depth': 1}]}
mdast.ast_to_html(ast)
# -> '<h1>title</h1>'
```

### Converting from mdast JSON to HTML

```python
import mdast

json_str = '{"type": "root", "children": [{"type": "heading", "children": [{"type": "text", "value": "title"}], "depth": 1}]}'
mdast.json_to_html(json_str)
# -> '<h1>title</h1>'
```

## Configuration Options

### Parsing Options

The `ParseOptions` class allows customization of Markdown parsing behavior:

```python
from mdast import ParseOptions

default_options = ParseOptions(
    # you can specify overwrites here
    frontmatter=True,
)
gfm_options = ParseOptions.gfm()
mdx_options = ParseOptions.mdx()
```

### Markdown Generation Options

The `MarkdownOptions` class customizes how Markdown is generated:

```python
from mdast import MarkdownOptions

options = MarkdownOptions(fences=False, emphasis="_")
```

Note that the Markdown generation of wooorm/markdown-rs does not support MDX or GFM generation.


### HTML Generation Options

The `GenerationOptions` class customizes how HTML is generated:

```python
from mdast import GenerationOptions

options = GenerationOptions(allow_dangerous_html=True)
```

## License

This project is licensed under the MIT License.

