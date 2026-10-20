import json


def library_at(path):
    if path is None:
        raise ValueError("reference: notes/reference-library.json must be inside this studio")
    try:
        library = json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        raise ValueError("reference: missing notes/reference-library.json") from None
    if not isinstance(library, list) or any(
        not isinstance(entry, dict)
        or any(not isinstance(entry.get(key), str) for key in ("id", "title", "text"))
        or not isinstance(entry.get("keywords"), list)
        or any(not isinstance(word, str) for word in entry["keywords"])
        for entry in library
    ):
        raise ValueError("reference: invalid notes/reference-library.json")
    seen = set()
    for entry in library:
        if entry["id"] in seen:
            raise ValueError(f'reference: notes/reference-library.json has duplicate id "{entry["id"]}"; give each entry its own id')
        seen.add(entry["id"])
    return library


def reference_list(path, params):
    if params:
        raise ValueError("reference_list: takes no arguments")
    return {"references": [{"id": entry["id"], "title": entry["title"]} for entry in library_at(path)]}


def reference(path, params):
    if set(params) - {"query", "id", "page"}:
        raise ValueError("reference: unknown parameter")
    if ("query" in params) == ("id" in params):
        raise ValueError("reference: give exactly one of query or id")
    if "query" in params:
        if not isinstance(params["query"], str) or not params["query"].strip():
            raise ValueError("reference: query must be a nonblank string")
        if "page" in params:
            raise ValueError("reference: page goes with id")
    elif not isinstance(params["id"], str):
        raise ValueError("reference: id must be a string")
    page = params.get("page", 1)
    if isinstance(page, float) and page.is_integer():
        page = int(page)
    if isinstance(page, bool) or not isinstance(page, int) or page < 1:
        raise ValueError("reference: page must be an integer starting at 1")
    library = library_at(path)
    if "query" in params:
        words = params["query"].lower().split()
        matches = []
        for entry in library:
            text = (entry["title"] + " " + " ".join(entry["keywords"]) + " " + entry["text"]).lower()
            if all(word in text for word in words):
                matches.append({"id": entry["id"], "title": entry["title"]})
        return {"matches": matches} if matches else {"matches": [], "message": "No matching references."}
    entry = next((entry for entry in library if entry["id"] == params["id"]), None)
    if entry is None:
        raise ValueError("reference: unknown id")
    pages, chars, size = [], [], 0
    for char in entry["text"]:
        width = len(char.encode("utf-8"))
        if size + width > 4000:
            pages.append("".join(chars))
            chars, size = [], 0
        chars.append(char)
        size += width
    if chars or not pages:
        pages.append("".join(chars))
    if page > len(pages):
        raise ValueError(f"reference: pages run from 1 to {len(pages)}")
    return {"id": entry["id"], "title": entry["title"], "page": page, "pages": len(pages),
            "next_page": page + 1 if page < len(pages) else None, "text": pages[page - 1]}
