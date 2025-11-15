#!/usr/bin/env python3
"""
Script to make white background transparent in PNG images.
Requires: pip install pillow
"""
import sys
from pathlib import Path

try:
    from PIL import Image
except ImportError:
    print("Error: PIL (Pillow) not installed. Install with: pip3 install pillow")
    sys.exit(1)

def make_transparent(input_path, output_path, threshold=30):
    """Make white (or near-white) background transparent."""
    img = Image.open(input_path).convert('RGBA')

    # Get pixel data
    data = img.getdata()

    new_data = []
    for item in data:
        # Check if pixel is white or near-white
        if item[0] > 255 - threshold and item[1] > 255 - threshold and item[2] > 255 - threshold:
            # Make it transparent
            new_data.append((255, 255, 255, 0))
        else:
            new_data.append(item)

    img.putdata(new_data)
    img.save(output_path, 'PNG')
    print(f"✓ Created transparent PNG: {output_path}")
    print(f"  Size: {output_path.stat().st_size / 1024:.1f} KB")

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: python3 make_transparent.py <input.png> [output.png] [threshold]")
        sys.exit(1)

    input_file = Path(sys.argv[1])
    output_file = Path(sys.argv[2]) if len(sys.argv) > 2 else input_file.parent / f"{input_file.stem}_transparent.png"
    threshold = int(sys.argv[3]) if len(sys.argv) > 3 else 30

    if not input_file.exists():
        print(f"Error: File not found: {input_file}")
        sys.exit(1)

    make_transparent(input_file, output_file, threshold)
