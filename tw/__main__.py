from tw.loader import load_json
from tw.generated import *
from tw.generated import __all_colors__, __all_shades__


def cap1(x: str) -> str:
    "Capitalize the first letter"
    return x[0].upper() + x[1:]


class Tailwind:
    def __init__(self) -> None:
        self.data = load_json()

    def __call__(self, color: Color, shade: Shade):
        return self.data[color][shade]


tw = Tailwind()

for color in __all_colors__:
    for shade in __all_shades__:
        hex_code = tw(color, shade)
        hex_code = hex_code.upper().removeprefix("#")

        out_color = cap1(color) + shade

        print("\\definecolor{%s}{HTML}{%s}" % (out_color, hex_code))
