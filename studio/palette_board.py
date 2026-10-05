"""Readable studio chips from the painter's lossless physical palette board."""
import io
from PIL import Image


def board_palette_chips(im):
    """Read the physical board's exact labels and paint on its light surface."""
    if im.height % 230:
        return None
    alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_ .()-+,:;'=/#?"
    glyphs = ("75557 26227 71747 71317 55711 74717 74757 71122 75757 75717 "
              "25755 65656 34443 65556 74647 74644 34553 55755 72227 11152 55655 44447 "
              "57755 65555 25552 65644 25563 65655 34216 72222 55557 55552 55775 55255 55222 71247 "
              "00007 00000 00002 12221 42224 00700 02720 00024 02020 02024 22000 07070 11244 57575 71202").split()
    letters = dict(zip(glyphs, alphabet))
    px = im.load()

    def text_at(x, y):
        text = ""
        for i in range(61):
            key = "".join(str(sum((4 >> dx) for dx in range(3)
                                  if px[x + i * 4 + dx, y + dy] == (245, 242, 230)))
                          for dy in range(5))
            if key not in letters:
                return None
            text += letters[key]
        return text.rstrip()

    chips = []
    for cy in range(0, im.height, 230):
        for cx in range(0, 1000, 250):
            prefix = f"{len(chips) + 1}. "
            label = next((text for dy in (4, 5, 6)
                          if (text := text_at(cx + 3, cy + dy)) and text.startswith(prefix)), None)
            if label:
                chips.append({"name": label[len(prefix):],
                              "thick": "#%02x%02x%02x" % px[cx + 60, cy + 75],
                              "thin": "#%02x%02x%02x" % px[cx + 150, cy + 110]})
    return chips or None


def palette_board(data):
    try:
        im = Image.open(io.BytesIO(data)).convert("RGB")
        if im.width != 1000:
            return None
        return board_palette_chips(im)
    except (OSError, ValueError):
        return None
