#!/usr/bin/env python3
"""Fills every documented example with the output of the real jql binary."""

import json
import pathlib
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


def render(example: dict) -> str:
    failed = ' class="failed"' if example["failed"] else ""
    outcome = "Error" if example["failed"] else "Output"
    flags = " ".join(example.get("flags", []))
    invocation = f"jql {flags} '{example['query']}'" if flags else f"jql '{example['query']}'"

    return f"""<div class="card">
<h3>{escape(example['title'])}</h3>
<p class="note">{escape(example['note'])}</p>
<div class="query"><code>{escape(invocation)}</code><button type="button">copy</button></div>
<div class="io pair">
<div class="slot"><div class="label">Input</div><pre>{escape(example['input'])}</pre></div>
<div class="slot"><div class="label">{outcome}</div><pre{failed}>{escape(example['output'])}</pre></div>
</div>
</div>"""


def main() -> None:
    jql = binary()
    examples = json.loads(SOURCE.read_text())
    failures = []

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
    for section in dict.fromkeys(example["section"] for example in examples):
        cards = "\n".join(
            render(example) for example in examples if example["section"] == section
        )
        marker = f"<!--examples:{section}-->"
        if marker not in page:
            sys.exit(f"{marker} is missing from {TEMPLATE.name}")
        page = page.replace(marker, cards)

    page = page.replace("<!--version-->", f"<code>{escape(version)}</code>")
    TARGET.write_text(page)
    print(f"wrote {TARGET.relative_to(ROOT)} ({len(examples)} examples, {version})")


if __name__ == "__main__":
    main()
