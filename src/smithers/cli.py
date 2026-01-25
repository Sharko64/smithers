import sys

from .commands import create, hello

COMMANDS = {
    "hello": hello.run,
    "create": create.run,
}

def main():
    if len(sys.argv) < 2:
        print("Usage: uv run smithers <command>")
        sys.exit(1)

    cmd_name = sys.argv[1]
    if cmd_name in COMMANDS:
        COMMANDS[cmd_name]()
    else:
        print(f"Unknown command: {cmd_name}")
        sys.exit(1)
