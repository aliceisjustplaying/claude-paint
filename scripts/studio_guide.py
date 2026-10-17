import argparse
from pathlib import Path
import re


def rewrite(text, watercolor=False, critic=False, reference_library=False, notebook=False):
    if watercolor:
        opening = "The studio has five MCP tools: `paint`, `look`, `note`, `status` and `log`."
        replacement = "The studio has seven MCP tools: `paint`, `look`, `note`, `status`, `log`, `reference` and `reference_list`.\n`reference` searches the studio's reference library (`query: \"<words>\"`) and reads an entry by its `id`, a page at a time, following `next_page` until it is null; `reference_list` lists the id and title of every entry."
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
        "reference": "| `reference` | searches the studio's reference library: `query: \"<words>\"` lists the entries that hold all of those words, by id and title; `id: \"<an entry's id>\"` reads that entry a page at a time; follow `next_page` until it is null |",
        "reference_list": "| `reference_list` | lists the id and title of every entry in the studio's reference library |",
        "ask_critic": "| `ask_critic` | asks the critic to visit now. The critic looks at the painting and reads your journal and your note, then writes back what it would fix and why. Its notes are the reply and are saved as `notes/critic.md`. `note`: what you'd like the critic to know or look at; it stays in `notes/to_critic.md` for later visits until you leave another |",
        "notebook": "| `notebook` | adds to your notebook, `notebook.md`, which carries over from one painting to the next; to revise it, give the exact passage as `replaces` and `text` takes its place |",
    })
    tools = ["paint", "look", "probe", "note", "status", "log", "read"]
    if reference_library:
        tools += ["reference", "reference_list"]
    if critic:
        tools.append("ask_critic")
    if notebook:
        tools.append("notebook")
    table = "| tool | what it does |\n|---|---|\n" + "\n".join(rows[name] for name in tools)
    text = text.replace(tables[0], table + "\n", 1)
    text = text.replace("The first chunk is `canvas{}` (see [The canvas](#the-canvas)). `read` reads the files in this folder: your brief and\nyour notes. `read` on a folder lists what is in it.",
                        "The first chunk is `canvas{}` (see [The canvas](#the-canvas)).")
    return re.sub(r"^\| `ref: [^\n]*\n?", "", text, flags=re.MULTILINE)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("guide", type=Path)
    parser.add_argument("--watercolor", action="store_true")
    args = parser.parse_args()
    args.guide.write_text(rewrite(args.guide.read_text(), watercolor=args.watercolor,
                                 reference_library=(args.guide.parent / "reference-library.json").is_file()))


if __name__ == "__main__":
    main()
