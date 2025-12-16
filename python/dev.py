#!/usr/bin/env python3
"""
Development script for mob Python bindings

This script provides commands for building, testing, and developing the Python bindings.
"""

import argparse
import subprocess
import sys
from pathlib import Path

PYTHON_DIR = Path(__file__).parent
PROJECT_ROOT = PYTHON_DIR.parent


def run_command(cmd, cwd=None, check=True):
    """Run a shell command and return the result."""
    print(f"🔧 Running: {' '.join(cmd)}")
    result = subprocess.run(cmd, cwd=cwd or PROJECT_ROOT, check=check)
    return result.returncode == 0


def install(dev=False):
    """Install Python package."""
    import os

    # Check if in a virtual environment
    in_venv = (
        hasattr(sys, 'real_prefix') or
        (hasattr(sys, 'base_prefix') and sys.base_prefix != sys.prefix) or
        os.environ.get('VIRTUAL_ENV') is not None
    )

    if not in_venv:
        print("⚠️  Warning: Not running in a virtual environment!")
        print("📝 It's recommended to use a virtual environment to avoid conflicts.")
        print("\nTo create and activate a virtual environment with uv (recommended):")
        print("  uv venv")
        print("  source .venv/bin/activate  # On Unix/macOS")
        print("  # or")
        print("  .\\.venv\\Scripts\\activate  # On Windows")
        print("\nOr with standard venv:")
        print("  python3 -m venv .venv")
        print("  source .venv/bin/activate  # On Unix/macOS")
        print("\nThen run this command again.\n")

        response = input("Continue anyway? [y/N]: ").strip().lower()
        if response != 'y':
            print("❌ Installation cancelled.")
            sys.exit(0)
        print()

    print("📦 Installing mob Python package...")

    # Upgrade pip
    print("📦 Upgrading pip...")
    run_command([sys.executable, "-m", "pip", "install", "--upgrade", "pip"])

    # Install maturin
    print("📦 Installing maturin...")
    run_command([sys.executable, "-m", "pip", "install", "maturin"])

    # Install dev dependencies if requested
    if dev:
        print("📦 Installing development dependencies...")
        run_command([
            sys.executable, "-m", "pip", "install", "-r",
            str(PYTHON_DIR / "requirements-dev.txt")
        ])

    # Build and install package in development mode
    print("📦 Building and installing package...")
    run_command(["maturin", "develop", "--release"])

    print("✅ Installation complete!")


def build():
    """Build Python package wheel."""
    print("🔨 Building Python package...")
    run_command(["maturin", "build", "--release"])
    print("✅ Build complete! Check target/wheels/ for the .whl file")


def test(verbose=False):
    """Run Python tests (excludes integration tests by default)."""
    print("🧪 Running Python tests...")
    print("ℹ️  Integration tests are excluded (use 'pytest tests/ -m integration' to run them)")

    cmd = [sys.executable, "-m", "pytest", str(PYTHON_DIR / "tests"), "-v", "-m", "not integration"]
    if verbose:
        cmd.extend(["-vv", "-s"])

    success = run_command(cmd, check=False)

    if success:
        print("✅ Python tests passed!")
    else:
        print("❌ Python tests failed!")
        sys.exit(1)


def lint():
    """Run Python linters."""
    print("🔍 Linting Python code...")

    # Black check
    print("\n📝 Checking code formatting with black...")
    black_ok = run_command(["black", "--check", str(PYTHON_DIR)], check=False)

    # isort check
    print("\n📝 Checking import sorting with isort...")
    isort_ok = run_command(["isort", "--check-only", str(PYTHON_DIR)], check=False)

    # mypy check
    print("\n📝 Checking types with mypy...")
    mypy_ok = run_command([
        "mypy",
        str(PYTHON_DIR / "mob"),
        str(PYTHON_DIR / "tests")
    ], check=False)

    if black_ok and isort_ok and mypy_ok:
        print("\n✅ All linting checks passed!")
    else:
        print("\n⚠️  Some linting checks failed. Run 'python dev.py format' to fix formatting.")
        sys.exit(1)


def format_code():
    """Format Python code."""
    print("✨ Formatting Python code...")

    # Black format
    print("\n📝 Formatting code with black...")
    run_command(["black", str(PYTHON_DIR)])

    # isort format
    print("\n📝 Sorting imports with isort...")
    run_command(["isort", str(PYTHON_DIR)])

    print("\n✅ Code formatting complete!")


def run_examples():
    """Run Python examples."""
    print("🚀 Running Python examples...\n")

    examples = [
        ("Basic Query", "basic_query.py"),
        ("Account Query", "account_query.py"),
    ]

    for name, script in examples:
        print("=" * 60)
        print(f"📊 {name} Example")
        print("=" * 60)
        success = run_command(
            [sys.executable, str(PYTHON_DIR / "examples" / script)],
            check=False
        )
        if not success:
            print(f"⚠️  {name} example failed")
        print()

    print("✅ Examples complete!")


def clean():
    """Clean build artifacts."""
    print("🧹 Cleaning build artifacts...")

    import shutil

    dirs_to_remove = [
        PROJECT_ROOT / "target",
        PROJECT_ROOT / "build",
        PROJECT_ROOT / "dist",
        PYTHON_DIR / ".pytest_cache",
        PYTHON_DIR / ".mypy_cache",
    ]

    for dir_path in dirs_to_remove:
        if dir_path.exists():
            print(f"  Removing {dir_path}")
            shutil.rmtree(dir_path)

    # Remove __pycache__ directories
    for pycache in PYTHON_DIR.rglob("__pycache__"):
        print(f"  Removing {pycache}")
        shutil.rmtree(pycache)

    # Remove .pyc files
    for pyc in PYTHON_DIR.rglob("*.pyc"):
        print(f"  Removing {pyc}")
        pyc.unlink()

    # Remove egg-info
    for egg_info in PROJECT_ROOT.glob("*.egg-info"):
        print(f"  Removing {egg_info}")
        shutil.rmtree(egg_info)

    print("✅ Clean complete!")


def main():
    parser = argparse.ArgumentParser(
        description="Development script for mob Python bindings",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python dev.py install --dev    # Install with dev dependencies
  python dev.py test --verbose   # Run tests with verbose output
  python dev.py lint             # Check code quality
  python dev.py format           # Format code
  python dev.py examples         # Run example scripts
  python dev.py build            # Build wheel package
  python dev.py clean            # Clean artifacts
        """
    )

    subparsers = parser.add_subparsers(dest="command", help="Command to run")

    # Install command
    install_parser = subparsers.add_parser(
        "install",
        help="Install Python package in development mode"
    )
    install_parser.add_argument(
        "--dev",
        action="store_true",
        help="Install with development dependencies (pytest, black, etc.)"
    )

    # Build command
    subparsers.add_parser("build", help="Build Python wheel package")

    # Test command
    test_parser = subparsers.add_parser("test", help="Run Python tests")
    test_parser.add_argument(
        "--verbose", "-v",
        action="store_true",
        help="Verbose test output"
    )

    # Lint command
    subparsers.add_parser("lint", help="Run Python linters (black, isort, mypy)")

    # Format command
    subparsers.add_parser("format", help="Format Python code with black and isort")

    # Examples command
    subparsers.add_parser("examples", help="Run Python example scripts")

    # Clean command
    subparsers.add_parser("clean", help="Clean build artifacts")

    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        sys.exit(0)

    # Execute command
    try:
        if args.command == "install":
            install(dev=args.dev)
        elif args.command == "build":
            build()
        elif args.command == "test":
            test(verbose=args.verbose)
        elif args.command == "lint":
            lint()
        elif args.command == "format":
            format_code()
        elif args.command == "examples":
            run_examples()
        elif args.command == "clean":
            clean()
    except KeyboardInterrupt:
        print("\n\n⚠️  Interrupted by user")
        sys.exit(1)
    except Exception as e:
        print(f"\n❌ Error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
