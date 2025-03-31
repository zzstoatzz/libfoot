"""Rich display functionality for libfoot."""

from __future__ import annotations

from libfoot.types import PackageFootprintDict, PyPIMetadataDict


def import_rich():
    """Import rich modules or raise ImportError with helpful message."""
    try:
        from rich.console import Console  # type: ignore
        from rich.table import Table  # type: ignore

        return Console, Table
    except ImportError:
        raise ImportError(
            "Pretty display requires the `display` extra. "
            "For example, `uvx 'libfoot[display]'`"
        )


def display_analysis(analysis_data: PackageFootprintDict) -> None:
    """
    Display package analysis results in a nicely formatted output.

    Args:
        analysis_data: The dictionary returned by analyze_package
    """
    Console, Table = import_rich()
    console = Console()

    pkg = analysis_data["package"]
    console.print(
        f"[bold cyan]Analysis for {pkg['name']} v{pkg['version']}[/bold cyan]"
    )
    console.print(f"Total Size: [green]{analysis_data['total_size']:,}[/green] bytes")
    console.print(f"File Count: [green]{analysis_data['file_count']:,}[/green]")

    if pkg["dependencies"]:
        console.print("\n[bold]Dependencies:[/bold]")
        for dep in pkg["dependencies"]:
            console.print(f"  • {dep}")

    # Display file types table
    table_types = Table(title="File Types")
    table_types.add_column("Extension", style="dim")
    table_types.add_column("Count", justify="right")

    # Sort file types for consistent output
    sorted_types = sorted(analysis_data["file_types"].items())
    for ext, count in sorted_types:
        ext_display = f".{ext}" if ext != "unknown" else ext
        table_types.add_row(ext_display, f"{count:,}")

    console.print(table_types)

    # Display largest files table
    table_largest = Table(title="Largest Files")
    table_largest.add_column("Path", style="green")
    table_largest.add_column("Size (bytes)", justify="right")
    table_largest.add_column("Type", style="dim")

    for file_info in analysis_data["largest_files"]:
        table_largest.add_row(
            file_info["path"], f"{file_info['size']:,}", file_info["file_type"]
        )

    console.print(table_largest)


def display_metadata(metadata: PyPIMetadataDict) -> None:
    """
    Display package metadata in a nicely formatted output.

    Args:
        metadata: The dictionary returned by get_pypi_metadata
    """
    Console, Table = import_rich()
    console = Console()

    console.print(
        f"[bold cyan]PyPI Metadata for {metadata['name']} v{metadata['version']}[/bold cyan]"
    )
    console.print(f"Summary: {metadata['summary']}")

    if metadata["release_url"]:
        console.print(f"Homepage: {metadata['release_url']}")

    if metadata["requires_python"]:
        console.print(f"Requires Python: {metadata['requires_python']}")

    if metadata["package_size"]:
        console.print(
            f"Package Size: [green]{metadata['package_size']:,}[/green] bytes"
        )

    if metadata["requires_dist"]:
        console.print("\n[bold]Dependencies:[/bold]")
        for dep in metadata["requires_dist"]:
            console.print(f"  • {dep}")
