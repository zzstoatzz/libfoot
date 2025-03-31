"""
Example script demonstrating libfoot's rich display functionality.

To run:
    uv run --extra display examples/rich_demo.py [package_name] [version]

Example:
    uv run --extra display examples/rich_demo.py torch 2.3.1
"""

import sys

from libfoot import analyze_package

try:
    from libfoot.display import display_analysis

    has_rich = True
except ImportError as e:
    has_rich = False
    print(
        "Pretty display requires the `display` extra. "
        "For example, `uv run --extra display examples/rich_demo.py`"
        f"\n{e}"
    )
    sys.exit(1)


def main():
    if len(sys.argv) < 2:
        print("Usage: python examples/rich_demo.py [package_name] [version]")
        sys.exit(1)

    package_name = sys.argv[1]
    version = sys.argv[2] if len(sys.argv) > 2 else None

    print(f"Analyzing {package_name}{' v' + version if version else ''}...\n")

    print("Analyzing package...")
    analysis = analyze_package(package_name, version)
    display_analysis(analysis)


if __name__ == "__main__":
    main()
