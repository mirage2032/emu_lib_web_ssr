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
    success: bool
    stdout: bytes
    stderr: bytes
    b64data: bytes

def compile_str(data: str) -> CompileResult:
    with TemporaryDirectory() as temp_dir:
        with open(f"{temp_dir}/main.c", "w") as f:
            f.write(data)
        command = ["zcc", "+z80", "-o", "main.bin", "-create-app", "main.c", "-compiler=sdcc", "-startup=0", "-lm"]
        result = subprocess.run(
            command,
            cwd=temp_dir,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        if result.returncode != 0:
            return CompileResult(success=False,stdout=base64.b64encode(result.stdout),stderr=base64.b64encode(result.stderr),b64data= b"")
        with open(f"{temp_dir}/main.bin", "rb") as f:
            return CompileResult(success=True,stdout=base64.b64encode(result.stdout),stderr=base64.b64encode(result.stderr), b64data=base64.b64encode(f.read()))

class CompileDataModel(BaseModel):
    data: str


@app.post("/compile")
def compile_data(item: CompileDataModel):
    return compile_str(item.data)