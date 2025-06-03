from itertools import product
from random import choice
import string
from typing import Optional
import common
from enum import IntFlag
import pytest

def setup_module(module):
    common.start_redis()


def teardown_module(module):
    common.shutdown_redis()

def setup_function(function):
    if common.g.name in common.client.list_graphs():
        common.g.delete()


class Type(IntFlag):
    NULL = 1
    BOOL = 2
    INT = 4
    FLOAT = 8
    STRING = 16
    LIST = 32
    MAP = 64
    NODE = 128
    RELATIONSHIP = 256
    PATH = 512

all_types = [
    Type.NULL,
    Type.BOOL,
    Type.INT,
    Type.FLOAT,
    Type.STRING,
    Type.LIST
]

def random_value(type):
    if type == Type.NULL:
        return "null"
    elif type == Type.BOOL:
        return choice(["true", "false"])
    elif type == Type.INT:
        return str(choice(range(-1000, 1000)))
    elif type == Type.FLOAT:
        return str(choice(list(map(lambda x: x / 10, range(-1000, 1000)))))
    elif type == Type.STRING:
        return f"\"{''.join(choice(string.ascii_letters + string.digits) for _ in range(choice(range(0, 100))))}\""
    elif type == Type.LIST:
        return str([random_value(choice(all_types)) for _ in range(choice(range(0, 10)))])
    elif isinstance(type, Optional):
        return choice([random_value(type.__args__[0]), None])
    return None

def is_valid_type(actual_type, expected_type):
    if isinstance(expected_type, Type):
        return actual_type & expected_type
    return actual_type & expected_type.__args__[0]

def validate_function(func, expected_args_type):
    args_type = []
    for _ in range(len(expected_args_type)):
        args_type.append(all_types)
    for args_type in product(*args_type):
        args = [x for x in [random_value(arg_type) for arg_type in args_type] if x is not None]
        is_unexpected = False
        for (arg_type, expected_type) in zip(args_type, expected_args_type):
            if not is_valid_type(arg_type, expected_type):
                is_unexpected = True
                break
        if not is_unexpected:
            args.append("null")
            query = f"RETURN {func}({', '.join(args)})"
            try:
                common.g.query(query)
                assert False
            except Exception as e:
                most = len(expected_args_type)
                assert f"Received {len(args)} arguments to function '{func}', expected at most {most}" in str(e)
            least = len([x for x in expected_args_type if isinstance(x, Type)])
            while len(args) >= least:
                args.pop()
            query = f"RETURN {func}({', '.join(args)})"
            try:
                common.g.query(query)
                assert False
            except Exception as e:
                least = len([x for x in expected_args_type if isinstance(x, Type)])
                assert f"Received {len(args)} arguments to function '{func}', expected at least {least}" in str(e)
        else:
            query = f"RETURN {func}({', '.join(args)})"
            try:
                common.g.query(query)
                assert False
            except Exception as e:
                assert True

def test_functions():
    validate_function("tointeger", [Type.STRING | Type.BOOL | Type.INT | Type.FLOAT | Type.NULL])
    validate_function("tofloat", [Type.STRING | Type.INT | Type.FLOAT | Type.NULL])
    validate_function("tostring", [Type.STRING | Type.INT | Type.BOOL | Type.NULL])
    validate_function("size", [Type.LIST | Type.STRING | Type.NULL])
    validate_function("head", [Type.LIST | Type.NULL])
    validate_function("last", [Type.LIST | Type.NULL])
    validate_function("tail", [Type.LIST | Type.NULL])
    validate_function("reverse", [Type.LIST | Type.STRING | Type.NULL])
    validate_function("substring", [Type.STRING | Type.NULL, Type.INT, Optional[Type.INT]])
    validate_function("split", [Type.STRING | Type.NULL, Type.STRING | Type.NULL])
    validate_function("tolower", [Type.STRING | Type.NULL])
    validate_function("toupper", [Type.STRING | Type.NULL])
    validate_function("replace", [Type.STRING | Type.NULL, Type.STRING | Type.NULL, Type.STRING | Type.NULL])
    validate_function("left", [Type.STRING | Type.NULL, Type.INT | Type.NULL])
    validate_function("ltrim", [Type.STRING | Type.NULL])
    validate_function("right", [Type.STRING | Type.NULL, Type.INT | Type.NULL])
    validate_function("string.join", [Type.LIST | Type.NULL, Optional[Type.STRING | Type.NULL]])
    validate_function("abs", [Type.INT | Type.FLOAT | Type.NULL])
    validate_function("ceil", [Type.INT | Type.FLOAT | Type.NULL])
    validate_function("exp", [Type.INT | Type.FLOAT | Type.NULL])
    validate_function("floor", [Type.INT | Type.FLOAT | Type.NULL])
    validate_function("log", [Type.INT | Type.FLOAT | Type.NULL])
    validate_function("log10", [Type.INT | Type.FLOAT | Type.NULL])
    validate_function("pow", [Type.INT | Type.FLOAT | Type.NULL, Type.INT | Type.FLOAT | Type.NULL])
    validate_function("round", [Type.INT | Type.FLOAT | Type.NULL])
    validate_function("sign", [Type.INT | Type.FLOAT | Type.NULL])
    validate_function("sqrt", [Type.INT | Type.FLOAT | Type.NULL])
    validate_function("range", [Type.INT, Type.INT, Optional[Type.INT]])
    validate_function("keys", [Type.MAP | Type.NULL])
    validate_function("toBoolean", [Type.BOOL | Type.STRING | Type.INT | Type.NULL])

@pytest.mark.extra
def test_extra_functions():
    validate_function("string.matchRegEx", [Type.STRING | Type.NULL, Type.STRING | Type.NULL])
    validate_function("string.replaceRegEx", [Type.STRING | Type.NULL, Type.STRING | Type.NULL, Type.STRING | Type.NULL])