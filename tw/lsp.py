from tw.loader import load_json, path
import sys

__cwd__ = path.dirname(__file__)

with open(path.join(__cwd__, "generated.py"), "w") as sys.stdout:
    data = load_json()

    colors = list(data.keys())
    print("Color = Literal%s" % colors)

    print("Shade = Literal%s" % list(data[colors[0]].keys()))

    print("Tailwind = dict[Color, dict[Shade, str]]")
