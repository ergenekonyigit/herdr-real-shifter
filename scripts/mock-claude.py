#!/usr/bin/env python3
import sys
import os

def set_title(title):
    sys.stdout.write(f"\033]0;{title}\007")
    sys.stdout.flush()

def main():
    set_title("claude (realshifter-test)")
    
    current_model = "claude-3-7-sonnet"
    
    print("\033[1;36m")
    print(" ╭─────────────────────────────────────────────────────────────╮")
    print(" │  🧠 Claude Code v2.1.235 [Mock Mode - RealShifter Testing]  │")
    print(" ╰─────────────────────────────────────────────────────────────╯")
    print("\033[0m")
    print(f" Current Model: \033[1;32m{current_model}\033[0m")
    print(" Ready for prompt or gear shifts (Gears 1-6 & Reverse R)...\n")

    while True:
        try:
            sys.stdout.write(f"\033[1;35mclaude [{current_model}] ❯ \033[0m")
            sys.stdout.flush()
            
            line = sys.stdin.readline()
            if not line:
                break
            
            cmd = line.strip()
            if not cmd:
                continue
            
            if cmd == "exit" or cmd == "quit":
                break
            elif cmd.startswith("/model"):
                parts = cmd.split(maxsplit=1)
                if len(parts) > 1:
                    current_model = parts[1]
                    print(f"\033[1;32m✓ Model switched to:\033[0m {current_model}")
                else:
                    print("\033[1;33mAvailable models: sonnet, haiku, opus, sonnet --thinking\033[0m")
            elif cmd.startswith("claude "):
                sub = cmd.replace("claude ", "")
                if "--model" in sub:
                    current_model = sub.replace("--model", "").strip()
                    print(f"\033[1;32m✓ Model switched to:\033[0m {current_model}")
                else:
                    print(f"\033[1;34m[Claude Command]: {cmd}\033[0m")
            else:
                print(f"\033[1;34m[Claude Prompt Received]:\033[0m {cmd}")
                
        except KeyboardInterrupt:
            print("\n\033[1;33m↺ Prompt interrupted / reset.\033[0m")
            continue
        except EOFError:
            break

if __name__ == "__main__":
    main()
