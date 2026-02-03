from tw.loader import load_json, path
import sys

__cwd__ = path.dirname(__file__)

with open(path.join(__cwd__, "generated.py"), "w") as sys.stdout:
    data = load_json()

    print("from typing import Literal")

    colors = list(data.keys())
    print("Color = Literal%s" % colors)
    print("__all_colors__: list[Color] = %s" % colors)

    print("Shade = Literal%s" % list(data[colors[0]].keys()))
    print("__all_shades__: list[Shade] = %s" % list(data[colors[0]].keys()))
