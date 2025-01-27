#!/usr/bin/env python3


import os
import subprocess

input_dir = "data/tiny-set/"
output_dir = "results/tiny-set/"
binary_path = "target/release/gtww"

os.makedirs(output_dir, exist_ok=True)

for filename in os.listdir(input_dir):
    if filename.endswith(".gr"):
        input_path = os.path.join(input_dir, filename)
        output_base = os.path.join(output_dir, filename)
        output_path = f"{output_base}.tww"
        log_path = f"{output_base}.log"

        with open(input_path, "rb") as stdin, open(output_path, "wb") as stdout:
            process = subprocess.Popen(
                [binary_path],
                stdin=stdin,
                stdout=stdout,
                stderr=subprocess.PIPE,
            )
            _, stderr = process.communicate()

            if stderr:
                with open(log_path, "wb") as log_file:
                    log_file.write(stderr)

