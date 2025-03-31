"""
Example script demonstrating libfoot's basic functionality.

The analogous CLI command is:

```bash
libfoot analyze requests
```
"""

import sys

from libfoot import analyze_package
from libfoot.display import display_analysis


def main():
    if len(sys.argv) < 2:
        print("Usage: python examples/demo.py [package_name] [version]")
        sys.exit(1)

    package_name = sys.argv[1]
    version = sys.argv[2] if len(sys.argv) > 2 else None

    print(f"Analyzing {package_name}{' v' + version if version else ''}...\n")

    print("Analyzing package...")
    analysis = analyze_package(package_name, version)
    display_analysis(analysis)


if __name__ == "__main__":
    main()
