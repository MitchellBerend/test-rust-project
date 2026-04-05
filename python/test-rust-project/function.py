from test_rust_project import add_numbers


def rust_add_numbers(a: int, b: int) -> int:
    if not isinstance(a, int) or not isinstance(b, int):
        raise TypeError("Inputs must be integers")

    return add_numbers(a, b)
