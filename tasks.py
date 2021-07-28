"""
Tasks for maintaining the project.
Execute 'invoke --list' for guidance on using Invoke
"""
import shutil
import pprint
import os

from invoke import task
import webbrowser
from pathlib import Path

Path().expanduser()

WASM_MODULE_NAME = "effectrack"

ROOT_DIR = Path(__file__).parent
WASM_BUILD_DIR = ROOT_DIR / "pkg"
BINARYEN_DIR = ROOT_DIR / "binaryen"
WABT_DIR = ROOT_DIR / "wabt"
WWW_DIR = ROOT_DIR / "www"
WWW_PUBLIC_DIR = WWW_DIR / "public"
WWW_PUBLIC_WASM_DIR = WWW_PUBLIC_DIR / "wasm"
WASM_SOURCE_DIR = ROOT_DIR / WASM_MODULE_NAME
WASM_TARGETS = [WASM_SOURCE_DIR] + [WASM_SOURCE_DIR / p for p in ["bpm-detection"]]


def _delete_file(file):
    try:
        file.unlink(missing_ok=True)
    except TypeError:
        # missing_ok argument added in 3.8
        try:
            file.unlink()
        except FileNotFoundError:
            pass


@task(help={"check": "Checks if source is formatted without applying changes"})
def format(c, check=False):
    """Format code"""
    python_dirs_string = " ".join(PYTHON_DIRS)
    black_options = "--diff" if check else ""
    c.run("pipenv run black {} {}".format(black_options, python_dirs_string))
    isort_options = "--check-only" if check else ""
    c.run("pipenv run isort {} {}".format(isort_options, python_dirs_string))


@task
def pack(c, upgrade=False):
    """Compile, pack and upgrade the wasm module package"""
    # os.environ["RUSTFLAGS"] = "-Ctarget-feature=+simd128"
    for p in WASM_TARGETS:
        c.run("wasm-pack build --target no-modules --release {}".format(p), pty=True)
        c.run("mkdir -p {}".format(WWW_PUBLIC_WASM_DIR / p.name))
        c.run("rm -rf {}".format(WWW_PUBLIC_WASM_DIR / p.name))
        c.run("cp -R {} {}".format(p / "pkg", WWW_PUBLIC_WASM_DIR / p.name))
        # if upgrade:
        # c.run("yarn --cwd {} upgrade {}".format(WWW_DIR, p.name), pty=True)
    # os.environ["RUSTFLAGS"] = ""


@task
def to_wav(c, audio_file, output_file):
    """Convert audio file to wav for easy streaming with wave"""
    audio_file = Path(audio_file)
    output_file = Path(output_file)

    # validate audio input file
    valid_audio_formats = [".mp3"]
    if not (audio_file.exists() and audio_file.suffix.lower() in valid_audio_formats):
        raise ValueError(
            "{} is not a valid audio file (must be one of {})".format(
                audio_file, ",".join(valid_audio_formats)
            )
        )

    # validate audio output file
    if not (output_file.suffix.lower() == ".wav"):
        raise ValueError("{} is not a wav file".format(output_file))

    # create the path to the output
    output_file.parent.mkdir(parents=True, exist_ok=True)

    # convert using ffmpeg
    c.run("ffmpeg -i {} {}".format(audio_file, output_file))


@task
def lint(c):
    """Lint code"""
    c.run("cargo clippy {}".format(WASM_SOURCE_DIR))


@task
def install_wasm_opt(c):
    """Install binaryen to optimize the web assembly bundle"""
    if not BINARYEN_DIR.is_dir():
        c.run("git clone git@github.com:WebAssembly/binaryen.git")
    c.run("cd binaryen && git pull")
    c.run("mkdir -p binaryen/build")
    c.run("cd binaryen/build && cmake .. && make -j && sudo make install")


@task
def install_wabt(c):
    """Install web assembly binary toolkit"""
    if not WABT_DIR.is_dir():
        c.run("git clone git@github.com:WebAssembly/wabt.git")
    c.run("cd wabt && git pull && git submodule update --init")
    c.run("mkdir -p wabt/build")
    c.run("cd wabt/build && cmake .. && make -j && sudo make install")


@task
def install_twiggy(c):
    """Install twiggy to inspect the generated wasm source sizes"""
    c.run("cargo install twiggy")


@task
def optimize_wasm(c, strip=True):
    """Optimize the wasm module"""
    c.run("wasm-opt {} -O4 -o {}".format(WASM_MODULE, WASM_MODULE))
    if strip:
        c.run("wasm-strip {}".format(WASM_MODULE))


@task
def inspect_wasm(c):
    """Inspect the wasm module"""
    c.run("twiggy top {}".format(WASM_MODULE))


# @task
# def test(c, min_coverage=None):
#     """Run tests"""
#     pytest_options = "--cov-fail-under={}".format(min_coverage) if min_coverage else ""
#     c.run("pipenv run pytest --cov={} {}".format(WASM_SOURCE_DIR, pytest_options))


# @task
# def install_hooks(c):
#     """Install pre-commit hooks"""
#     c.run("pipenv run pre-commit install -t pre-commit")
#     c.run("pipenv run pre-commit install -t pre-push")


# @task
# def pre_commit(c):
#     """Run all pre-commit checks"""
#     c.run("pipenv run pre-commit run --all-files")


# @task
# def clean_build(c):
#     """Clean up files from package building"""
#     c.run("rm -fr build/")
#     c.run("rm -fr dist/")
#     c.run("rm -fr .eggs/")
#     c.run("find . -name '*.egg-info' -exec rm -fr {} +")
#     c.run("find . -name '*.egg' -exec rm -f {} +")


# @task(pre=[clean_build])
# def clean(c):
#     """Runs all clean sub-tasks"""
#     pass


# @task(pre=[clean])
# def release(c):
#     """Make a release of the python package to pypi"""
#     c.run("twine upload dist/*")
