from __future__ import annotations

import argparse
import json
import sys

from libfoot import analyze_package, get_pypi_metadata
from libfoot.display import display_analysis, display_metadata


def create_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Analyze the footprint of a Python package", prog="libfoot"
    )

    subparsers = parser.add_subparsers(dest="command", required=True)

    analyze_parser = subparsers.add_parser(
        "analyze", help="Analyze package size and contents"
    )
    analyze_parser.add_argument("package", help="Package name")
    analyze_parser.add_argument("--version", "-v", help="Package version (optional)")
    analyze_parser.add_argument(
        "--json", action="store_true", help="Output in JSON format"
    )

    metadata_parser = subparsers.add_parser("metadata", help="Get package metadata")
    metadata_parser.add_argument("package", help="Package name")
    metadata_parser.add_argument("--version", "-v", help="Package version (optional)")
    metadata_parser.add_argument(
        "--json", action="store_true", help="Output in JSON format"
    )

    return parser


def main(args: list[str] | None = None) -> int:
    parser = create_parser()

    if args is None:
        args = sys.argv[1:]

    parsed_args = parser.parse_args(args)

    try:
        if parsed_args.command == "analyze":
            result = analyze_package(parsed_args.package, parsed_args.version)

            if parsed_args.json:
                print(json.dumps(result, indent=2))
            else:
                display_analysis(result)

        elif parsed_args.command == "metadata":
            result = get_pypi_metadata(parsed_args.package, parsed_args.version)

            if parsed_args.json:
                print(json.dumps(result, indent=2))
            else:
                display_metadata(result)

        return 0

    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
