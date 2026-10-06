"""Exercise the plugin in clean Neovim instances on every supported OS."""
import os
from pathlib import Path
import subprocess
import tempfile


def main():
    repo = Path(__file__).resolve().parent.parent
    suffix = ".exe" if os.name == "nt" else ""
    binary = repo / "target" / "release" / f"study{suffix}"
    if not binary.is_file():
        raise SystemExit("Run cargo build --release --locked first")
    for test in sorted((repo / "tests" / "nvim").glob("*.lua")):
        with tempfile.TemporaryDirectory(prefix="study portable ") as temporary:
            environment = dict(os.environ, STUDY_TEST_ROOT=temporary,
                               STUDY_TEST_BINARY=str(binary))
            subprocess.run([os.environ.get("NVIM", "nvim"), "--headless", "-u", "NONE",
                            "-l", str(test)], cwd=repo, env=environment, check=True, timeout=30)
        print(f"PASS {test.name}", flush=True)


if __name__ == "__main__":
    main()
