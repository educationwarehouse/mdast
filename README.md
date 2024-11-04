# mdast.py

Simple Python bindings for the [mdast](https://github.com/syntax-tree/mdast) functionality of [wooorm/markdown-rs](https://github.com/wooorm/markdown-rs/)

## Installation
```bash
pip install mdast
```

If you're on x86-64/AMD64 or arm64/aarch64 you can install this package without having Rust on your system.
For other platforms, the rust toolchain is required to build the binary dependencies.

## Usage

### Converting from markdown to mdast's json format

```python
import mdast

mdast.md_to_json("# title")
# -> {"type":"root","children":[{"type":"heading","children":[{"type":"text","value":"title","position":{"start":{"line":1,"column":3,"offset":2},"end":{"line":1,"column":8,"offset":7}}}],"position":{"start":{"line":1,"column":1,"offset":0},"end":{"line":1,"column":8,"offset":7}},"depth":1}],"position":{"start":{"line":1,"column":1,"offset":0},"end":{"line":1,"column":8,"offset":7}}}
```

### Converting from mdast to markdown

```python
import mdast

mdast.json_to_md('{"type":"root","children":[{"type":"heading","children":[{"type":"text","value":"title","position":{"start":{"line":1,"column":3,"offset":2},"end":{"line":1,"column":8,"offset":7}}}],"position":{"start":{"line":1,"column":1,"offset":0},"end":{"line":1,"column":8,"offset":7}},"depth":1}],"position":{"start":{"line":1,"column":1,"offset":0},"end":{"line":1,"column":8,"offset":7}}}')
# -> '# title\n'
```
