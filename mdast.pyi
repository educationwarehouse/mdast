
def md_to_json(md: str) -> str:
    """
    Convert markdown to an mdast json representation.

    Args:
        md (str): Markdown

    Returns:
        json string, because that's way easier than building a python object from rust
    """

def json_to_md(json: str) -> str:
    """
    Convert mdast json back to markdown.

    Args:
        json (str): mdast representation, in string format.

    Returns:
        markdown
    """