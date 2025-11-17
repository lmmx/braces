#!/usr/bin/env python3
import sys

def pretty_braces(expr):
    lines = []
    stack = []  # tracks column of each '{'
    i = 0
    line = ""
    while i < len(expr):
        c = expr[i]
        if c == "{":
            line += "{"
            lines.append(line)
            stack.append(len(line))  # record column of '{'
            line = " " * stack[-1]   # start new line at brace column
            i += 1
        elif c == "}":
            if line.strip():
                lines.append(line.rstrip(","))
            line = " " * stack.pop() + "}"
            lines.append(line)
            line = " " * (stack[-1] if stack else 0)
            i += 1
        elif c == ",":
            line += ","
            lines.append(line)
            line = " " * (stack[-1] if stack else 0)
            i += 1
        else:
            line += c
            i += 1
    if line.strip():
        lines.append(line)
    return "\n".join(lines)

if __name__ == "__main__":
    if not sys.stdin.isatty():
        expr = sys.stdin.read().strip()
    elif len(sys.argv) > 1:
        expr = sys.argv[1]
    else:
        print("Usage: ppb.py [BRACES_EXPR] or pipe into stdin", file=sys.stderr)
        sys.exit(1)

    print(pretty_braces(expr))
