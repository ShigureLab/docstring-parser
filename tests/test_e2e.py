import inspect

from docstring_parser import parse


def prepare_doc(docstring: str) -> str:
    return "\n" + inspect.cleandoc(docstring)


def test_parse_args():
    docstring = """
    Args:
        arg1 (int): Description of arg1
        arg2 (str, optional): Description of arg2
    """
    docstring = prepare_doc(docstring)
    parsed_doc = parse(docstring)
    print(parsed_doc)
    # breakpoint()
    raise
