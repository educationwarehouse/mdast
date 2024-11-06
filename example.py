import mdast

if __name__ == '__main__':
    value = "#hello"
    json = mdast.md_to_json(value)
    md = mdast.json_to_md(json)
    print(value)
    print(md)
