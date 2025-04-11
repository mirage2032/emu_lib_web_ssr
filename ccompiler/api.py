import os
from fastapi import FastAPI
from pydantic import BaseModel
from pydantic.dataclasses import dataclass
from tempfile import TemporaryDirectory
import base64
import subprocess

app = FastAPI()

@dataclass
class CompileResult:
    rc: int
    b64stdout: bytes
    b64stderr: bytes
    b64data: bytes

FILENAME = "main"

def compile_str(b64data_in: str) -> CompileResult:
    with TemporaryDirectory() as temp_dir:
        with open(f"{temp_dir}/{FILENAME}.c", "w") as f:
            data_in = base64.b64decode(b64data_in).decode("utf-8")
            f.write(data_in)
        command = ["zcc", "+z80", "-vn", "-O3", "-startup=31", "-o", f"{FILENAME}.rom", "-create-app", f"{FILENAME}.c", "-compiler=sdcc","-zorg=0x0", "-lm"]
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
        return CompileResult(rc=result.returncode,b64stdout=base64.b64encode(result.stdout),b64stderr=base64.b64encode(result.stderr), b64data=base64.b64encode(data))

class CompileDataModel(BaseModel):
    b64data: str


@app.post("/compile") #Input and output are base64 encoded
def compile_data(item: CompileDataModel):
    return compile_str(item.b64data)