import os
from fastapi import FastAPI
from pydantic import BaseModel
from pydantic.dataclasses import dataclass
from tempfile import TemporaryDirectory, NamedTemporaryFile
import base64
import subprocess

app = FastAPI()


@dataclass
class CompileData:
    rc: int
    b64stdout: bytes
    b64stderr: bytes
    b64data: bytes


@dataclass
class FormatData:
    b64data: bytes


@dataclass
class SyntaxCheckData:
    rc: int
    b64stderr: bytes


FILENAME = "main"


def compile_str(b64data_in: str) -> CompileData:
    data_in = base64.b64decode(b64data_in).decode("utf-8")
    with TemporaryDirectory() as temp_dir:
        with open(f"{temp_dir}/{FILENAME}.c", "w") as f:
            f.write(data_in)
        command = ["zcc", "+z80", "-vn", "-O3", "-startup=31", "-o", f"{FILENAME}.rom", "-create-app", f"{FILENAME}.c",
                   "-compiler=sdcc", "-zorg=0x0", "-lm"]
        result = subprocess.run(
            command,
            cwd=temp_dir,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        data: bytes = b""
        try:
            with open(f"{temp_dir}/{FILENAME}.rom", "rb") as f:
                data = f.read()
        except  FileNotFoundError:
            data = b""
        return CompileData(rc=result.returncode, b64stdout=base64.b64encode(result.stdout),
                           b64stderr=base64.b64encode(result.stderr), b64data=base64.b64encode(data))


def format_str(b64data_in: str) -> FormatData:
    data_in = base64.b64decode(b64data_in).decode("utf-8")
    with NamedTemporaryFile(mode="w+", suffix=".c") as f:
        f.write(data_in)
        f.flush()
        command = [
            "indent",
            "-br",  # brace right (same line) for blocks like if, while
            "-brf",  # brace right for functions
            "-npsl",  # don't put return type on its own line
            "-npcs",  # no space between function name and (
            "-nut",  # no tabs
            "-i4",  # indent size 4
            "-cli4",  # continuation indent
            f.name
        ]
        result = subprocess.run(
            command,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        data = b""
        if result.returncode == 0:
            f.seek(0)
            data = f.read().encode("utf-8")
        return FormatData(b64data=base64.b64encode(data))


def syntax_check(b64data_in: str) -> SyntaxCheckData:
    data_in = base64.b64decode(b64data_in).decode("utf-8")
    with NamedTemporaryFile(mode="w+", suffix=".c") as f:
        f.write(data_in)
        f.flush()
        command = ["gcc", "-fsyntax-only", f.name]
        result = subprocess.run(
            command,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
        )
        return SyntaxCheckData(rc=result.returncode, b64stderr=base64.b64encode(result.stderr))


class RequestDataModel(BaseModel):
    b64data: str


@app.post("/compile")
def compile_data_endpoint(item: RequestDataModel):
    return compile_str(item.b64data)


@app.post("/format")
def format_data_endpoint(item: RequestDataModel):
    return format_str(item.b64data)


@app.post("/syntax_check")
def syntax_check_endpoint(item: RequestDataModel):
    return syntax_check(item.b64data)
