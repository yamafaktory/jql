#!/usr/bin/env python3
"""Fills every documented example with the output of the real jql binary."""

import json
import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
SOURCE = ROOT / "docs" / "examples.json"
TEMPLATE = ROOT / "docs" / "template.html"
TARGET = ROOT / "docs" / "index.html"


def binary() -> pathlib.Path:
    path = ROOT / "target" / "release" / "jql"
    if not path.exists():
        sys.exit(f"{path} is missing, run: cargo build --release -p jql")
    return path


def run(jql: pathlib.Path, example: dict) -> tuple[str, int]:
    command = [str(jql), *example.get("flags", [])]
    if example["query"]:
        command.append(example["query"])

    result = subprocess.run(
        command,
        input=example["input"].encode(),
        capture_output=True,
        env={"NO_COLOR": "1", "PATH": "/usr/bin:/bin"},
    )
    stream = result.stdout if result.returncode == 0 else result.stderr
    return stream.decode().rstrip("\n"), result.returncode


def escape(text: str) -> str:
    return (
        text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace('"', "&quot;")
    )


TOKEN = re.compile(
    r"""(?P<string>"(?:[^"\\]|\\.)*")"""
    r"""|(?P<number>-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)"""
    r"""|(?P<literal>true|false|null)"""
    r"""|(?P<punct>[{}\[\],:])"""
)


def highlight(text: str) -> str:
    """Wraps JSON tokens in spans. Anything unrecognised is passed through."""
    out = []
    index = 0

    for match in TOKEN.finditer(text):
        out.append(escape(text[index : match.start()]))
        kind = match.lastgroup
        value = match.group()

        if kind == "string":
            rest = text[match.end() :].lstrip()
            kind = "key" if rest.startswith(":") else "string"

        out.append(f'<span class="{kind}">{escape(value)}</span>')
        index = match.end()

    out.append(escape(text[index:]))
    return "".join(out)


def render(example: dict) -> str:
    failed = ' class="failed"' if example["failed"] else ""
    outcome = "Error" if example["failed"] else "Output"
    body = escape(example["output"]) if example["failed"] else highlight(example["output"])
    flags = " ".join(example.get("flags", []))
    invocation = f"jql {flags} '{example['query']}'" if flags else f"jql '{example['query']}'"

    if "fixture" in example:
        panes = f'''<div class="slot"><div class="label">{outcome}</div><pre{failed}>{body}</pre></div>'''
        io = "io single"
    else:
        panes = (
            f'''<div class="slot"><div class="label">Input</div><pre>{highlight(example["input"])}</pre></div>'''
            f'''\n<div class="slot"><div class="label">{outcome}</div><pre{failed}>{body}</pre></div>'''
        )
        io = "io"

    return f"""<div class="card">
<h3>{escape(example['title'])}</h3>
<p class="note">{escape(example['note'])}</p>
<div class="term"><span class="prompt">$</span><code>{escape(invocation)}</code><button type="button">copy</button></div>
<div class="{io}">
{panes}
</div>
</div>"""


def main() -> None:
    jql = binary()
    source = json.loads(SOURCE.read_text())
    fixtures = source["fixtures"]
    examples = source["examples"]
    failures = []

    for example in examples:
        if "fixture" in example:
            if example["fixture"] not in fixtures:
                sys.exit(f"unknown fixture {example['fixture']!r}")
            example["input"] = fixtures[example["fixture"]]

    for example in examples:
        output, code = run(jql, example)
        expected_error = example.get("expect") == "error"

        if expected_error and code == 0:
            failures.append(f"{example['title']}: expected an error, got a result")
        elif not expected_error and code != 0:
            failures.append(f"{example['title']}: {output}")

        example["output"] = output
        example["failed"] = code != 0

    if failures:
        sys.exit("\n".join(["examples that no longer behave as documented:", *failures]))

    version = subprocess.run(
        [str(jql), "--version"], capture_output=True, text=True
    ).stdout.strip()

    page = TEMPLATE.read_text()
    for name, document in fixtures.items():
        marker = f"<!--fixture:{name}-->"
        if marker not in page:
            sys.exit(f"{marker} is missing from {TEMPLATE.name}")
        page = page.replace(marker, f"<pre>{highlight(document)}</pre>")
    for section in dict.fromkeys(example["section"] for example in examples):
        cards = "\n".join(
            render(example) for example in examples if example["section"] == section
        )
        marker = f"<!--examples:{section}-->"
        if marker not in page:
            sys.exit(f"{marker} is missing from {TEMPLATE.name}")
        page = page.replace(marker, cards)

    page = page.replace("<!--version-->", f"<code>{escape(version)}</code>")
    if "<!--" in page.split("<script>")[0]:
        sys.exit("a placeholder was left unreplaced in the page")
    TARGET.write_text(page)
    print(f"wrote {TARGET.relative_to(ROOT)} ({len(examples)} examples, {version})")


if __name__ == "__main__":
    main()
