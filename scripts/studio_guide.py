import argparse
from pathlib import Path
import re


def rewrite(text, watercolor=False, reference_library=False, critic=False):
    if watercolor:
        opening = "The studio has five MCP tools: `paint`, `look`, `note`, `status` and `log`."
        replacement = "The studio has seven MCP tools: `paint`, `look`, `note`, `status`, `log`, `reference` and `reference_list`.\n`reference_list` lists the reference library's entries by `id` and title; `reference` with `query` finds the entries that hold all its words, and with `id` reads one a page at a time, following `next_page` until it is null."
        if opening not in text and replacement not in text:
            raise ValueError("the watercolor guide has no tool list")
        return text.replace(opening, replacement)
    tables = re.findall(r"^\| tool \| what it does \|\n(?:\|[^\n]*\n?)+", text, re.MULTILINE)
    if "| tool | what it does |" not in text:
        return re.sub(r"^\| `ref: [^\n]*\n?", "", text, flags=re.MULTILINE)
    if len(tables) != 1:
        raise ValueError("the guide must have one tool table")
    rows = {
        match[1]: match[0]
        for match in re.finditer(r"^\| `([^`]+)` \| .*\|$", tables[0], re.MULTILINE)
    }
    rows.update({
        "read": "| `read` | reads a file in this folder, such as your brief or your notes; on a folder, lists what is in it |",
        "reference": "| `reference` | searches the reference library or reads from it: `query: \"<words>\"` gives the `id` and title of each entry that holds all the words, in any case; `id: \"<id>\"` gives that entry's text a page at a time, with `page` to choose one and `next_page` naming the next (null on the last) |",
        "reference_list": "| `reference_list` | lists the reference library: each entry's `id` and title |",
        "ask_critic": "| `ask_critic` | asks the critic to visit now. The critic looks at the painting and reads your journal and your note, then writes back what it would fix and why; its notes are the reply and are saved as `notes/critic.md`. `note: \"<text>\"` is what you'd like the critic to know or look at; it stays in `notes/to_critic.md` for later visits until you leave another |",
    })
    tools = ["paint", "look"] + (["probe"] if "probe" in rows else []) + ["note", "status", "log"]
    if reference_library:
        tools += ["read", "reference", "reference_list"]
    if critic:
        tools.append("ask_critic")
    table = "| tool | what it does |\n|---|---|\n" + "\n".join(rows[name] for name in tools)
    text = text.replace(tables[0], table + "\n", 1)
    text = text.replace("The first chunk is `canvas{}` (see [The canvas](#the-canvas)). `read` reads the files in this folder: your brief and\nyour notes. `read` on a folder lists what is in it.",
                        "The first chunk is `canvas{}` (see [The canvas](#the-canvas)).")
    return re.sub(r"^\| `ref: [^\n]*\n?", "", text, flags=re.MULTILINE)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("guide", type=Path)
    parser.add_argument("--watercolor", action="store_true")
    parser.add_argument("--studio", type=Path)
    args = parser.parse_args()
    studio = args.studio or args.guide.parent.parent
    args.guide.write_text(rewrite(args.guide.read_text(), watercolor=args.watercolor,
                                 reference_library=(studio / "notes/reference-library.json").is_file()))


if __name__ == "__main__":
    main()
