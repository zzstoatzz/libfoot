"""
Example script demonstrating libfoot's rich display functionality.

To run:
    pip install libfoot[display]
    python rich_display.py [package_name] [version]

Example:
    python rich_display.py requests 2.31.0
"""

import sys

from libfoot import analyze_package, get_pypi_metadata

# Try to import the display functions
try:
    from libfoot.display import display_analysis, display_metadata

    HAS_RICH = True
except ImportError as e:
    HAS_RICH = False
    print(
        "Pretty display requires the `display` extra. "
        "For example, `uv run --extra display examples/rich_demo.py`"
        f"\n{e}"
    )
    sys.exit(1)


def main():
    # Get package name and optional version from command line
    if len(sys.argv) < 2:
        print("Usage: python rich_display.py [package_name] [version]")
        sys.exit(1)

    package_name = sys.argv[1]
    version = sys.argv[2] if len(sys.argv) > 2 else None

    print(f"Analyzing {package_name}{' v' + version if version else ''}...\n")

    # Get and display package metadata
    print("Fetching metadata...")
    metadata = get_pypi_metadata(package_name, version)
    display_metadata(metadata)

    print("\n" + "-" * 50 + "\n")

    # Analyze and display package footprint
    print("Analyzing package content...")
    analysis = analyze_package(package_name, version)
    display_analysis(analysis)


if __name__ == "__main__":
    main()
