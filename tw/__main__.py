from tw.loader import load_json

data = load_json()

color_names: list[str] = list(data.keys())
shades: list[str] = list(data[color_names[0]].keys())

for color_name in color_names:
    for shade in shades:
        hex_code = data[color_name][shade]
        print(hex_code)
