from os import path


def load_json() -> dict[str, dict[str, str]]:
    __root__ = path.dirname(path.dirname(__file__))

    with open(path.join(__root__, "source.json"), "r") as f:
        from json import load

        return load(f)
