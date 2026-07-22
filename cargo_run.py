import sys
import subprocess

SCRIPT_NAME = "cargo_run"

def main():
    # Validate input
    if len(sys.argv) < 2:
        print(f"Usage: python {SCRIPT_NAME}.py <project-name>")
        print(f"Example: python {SCRIPT_NAME}.py unwrap")
        sys.exit(1)
    
    project_name = sys.argv[1]
    
    # Build
    result = subprocess.run(["cargo", "build", "-p", project_name])
    if result.returncode != 0:
        sys.exit(1)
    
    # Test
    result = subprocess.run(["cargo", "test", "-p", project_name])
    if result.returncode != 0:
        sys.exit(1)
    
    print()
    print("SUCCESS")

if __name__ == "__main__":
    main()
