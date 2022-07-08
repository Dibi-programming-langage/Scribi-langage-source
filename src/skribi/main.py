#!/usr/bin/env python3
# *-* coding:utf-8 *-*

# ====================== #
# Skribi language module #
# ====================== #

# instance of Program class
from . import program
from . import skribi_file

program_instance = program.Program()

# shell's file
shell_file = skribi_file.SkribiFile(None, "shell")


def execute(code, file, path=None):
    """ Execute code in Skribi """
    print("Executing code in Skribi...")
    if file and path is not None:
        program_instance.analyse(skribi_file.SkribiFile(code, path))
    else:
        shell_file.set_content(code)
        program_instance.analyse(shell_file)
