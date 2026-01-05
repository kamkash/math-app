#!/usr/bin/env python3
"""
run_grun.py

Small helper script to run the ANTLR "grun" (TestRig) command in a bash shell with
the CLASSPATH environment variable set, matching the user's requested command.

Usage examples:
  # make it executable once
  chmod +x math-parser/tools/run_grun.py

  # run with defaults (uses ../antlr-4.13.2-complete.jar and block.tex)
  ./math-parser/tools/run_grun.py

  # override input file or classpath
  ./math-parser/tools/run_grun.py --input-file other_input.tex --classpath ../antlr-4.13.2-complete.jar

This script executes the command in a bash shell (via `bash -lc`) so the CLASSPATH
assignment before the `grun` call behaves the same as in an interactive shell.
"""

from __future__ import annotations

import argparse
import shlex
import subprocess
import sys
from pathlib import Path

G4_FILE_DEFAULT = "/Users/kamran/mathappws/math-app/math-parser/tools/grammars/LaTeX.g4"
ANTLR_JAR_DEFAULT = "/Users/kamran/mathappws/antlr4java/antlr-4.13.2-complete.jar"
INPUT_FILE_DEFAULT = "block.tex"


def build_command(
    classpath: str, grammar: str, rule: str, flags: str, input_file: str
) -> str:
    # Build a shell command that:
    # 1) copies the .g4 grammar file into the current directory
    # 2) runs the ANTLR tool to generate Java sources (visitor)
    # 3) compiles generated Java sources with javac
    # 4) runs grun (TestRig) with CLASSPATH set
    # Use && between steps so failures stop the sequence.

    grammar_g4 = f"{grammar}.g4"

    # Copy the grammar file into the current working directory. Use the configured G4_FILE_DEFAULT
    cp_cmd = f"cp -f {shlex.quote(G4_FILE_DEFAULT)} {shlex.quote(grammar_g4)}"

    # Run ANTLR tool to generate java sources. Use the provided classpath (typically the ANTLR jar).
    java_tool_cmd = f"java -cp {shlex.quote(classpath)} org.antlr.v4.Tool -visitor {shlex.quote(grammar_g4)}"

    # Compile all generated Java files.
    javac_cmd = f"javac -cp {shlex.quote(classpath)} *.java"

    # Prepare flags safely (preserve multiple flags)
    flags_tokens = shlex.split(flags or "")
    flags_part = " ".join(shlex.quote(t) for t in flags_tokens)

    # Final grun invocation with CLASSPATH including current dir and the ANTLR jar
    grun_cmd = f"CLASSPATH=.:{shlex.quote(classpath)} java org.antlr.v4.gui.TestRig {shlex.quote(grammar)} {shlex.quote(rule)} {flags_part} {shlex.quote(input_file)}"

    # Chain all commands so they run sequentially and abort on error
    full_cmd = " && ".join([cp_cmd, java_tool_cmd, javac_cmd, grun_cmd])
    return full_cmd


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Run ANTLR 'grun' (TestRig) in a bash shell with CLASSPATH set."
    )
    parser.add_argument(
        "--classpath",
        "-c",
        default=ANTLR_JAR_DEFAULT,
        help=f"Path to the ANTLR complete jar (default: {ANTLR_JAR_DEFAULT})",
    )
    parser.add_argument(
        "--grammar", "-g", default="LaTeX", help="Grammar name (default: LaTeX)"
    )
    parser.add_argument(
        "--rule", "-r", default="block", help="Start rule to invoke (default: block)"
    )
    parser.add_argument(
        "--flags",
        "-f",
        default="-tree -gui",
        help="Extra flags to pass to grun (default: '-tree -gui')",
    )
    parser.add_argument(
        "--input-file",
        "-i",
        default=INPUT_FILE_DEFAULT,
        help=f"Input file to feed to grun (default: {INPUT_FILE_DEFAULT})",
    )

    args = parser.parse_args(argv)

    input_path = Path(args.input_file)
    if not input_path.exists():
        print(
            f"Warning: input file '{args.input_file}' does not exist in current directory."
        )

    shell_cmd = build_command(
        args.classpath, args.grammar, args.rule, args.flags, args.input_file
    )

    full_cmd = ["bash", "-lc", shell_cmd]
    print(f"Running: {shell_cmd}")

    try:
        proc = subprocess.run(full_cmd)
        return proc.returncode
    except FileNotFoundError as e:
        print(
            "Error: bash or grun not found. Make sure Java and ANTLR grun (TestRig) are available in PATH."
        )
        print(e)
        return 2
    except KeyboardInterrupt:
        print("Interrupted by user")
        return 130


if __name__ == "__main__":
    raise SystemExit(main())
