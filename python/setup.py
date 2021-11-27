#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""The setup script."""

from setuptools import find_packages, setup
from setuptools_rust import Binding, RustExtension
from pathlib import Path

short_description = "No description has been added so far."

version = "0.1.1"

try:
    if (Path().parent / "README.md").is_file():
        import m2r

        long_description = m2r.parse_from_file(Path().parent / "README.md")
    else:
        raise AssertionError("missing README file")
except (ImportError, AssertionError):
    long_description = short_description

requirements = [
    "Click>=6.0",
]
build_requirements = [
    "setuptools-rust",
    "twine",
    "wheel",
    "cibuildwheel",
]
test_requirements = [
    "tox",
    "pytest",
    "pytest-cov",
    "pytest-xdist",
    "pytest-sugar",
    "mypy",
    "pyfakefs",
]
coverage_requirements = ["coverage"]
formatting_requirements = ["flake8", "black==21.8b0"]
tool_requirements = [
    "m2r",
    "invoke",
    "pre-commit",
    "bump2version",
]
dev_requirements = (
    requirements
    + build_requirements
    + test_requirements
    + coverage_requirements
    + formatting_requirements
    + tool_requirements
)

setup(
    author="romnn",
    author_email="contact@romnn.com",
    classifiers=[
        "Development Status :: 2 - Pre-Alpha",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Environment :: Console",
        "Operating System :: OS Independent",
        "Natural Language :: English",
        "Programming Language :: Python",
        "Programming Language :: Python :: 3 :: Only",
        "Programming Language :: Python :: 3.5",
        "Programming Language :: Python :: 3.6",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
    ],
    entry_points={
        "console_scripts": [
            "pydisco=discodisco.cli:main",
            "discodisco=discodisco.cli:main",
        ]
    },
    python_requires=">=3.6",
    install_requires=requirements,
    setup_requires=tool_requirements,
    tests_require=test_requirements,
    extras_require=dict(dev=dev_requirements, test=test_requirements),
    license="MIT",
    description=short_description,
    long_description=long_description,
    include_package_data=True,
    package_data={"discodisco": []},
    keywords="discodisco",
    name="discodisco",
    rust_extensions=[RustExtension("disco", binding=Binding.PyO3)],
    packages=find_packages(
        include=["discodisco"], exclude=["protoc", "disco-src/target", "target"]
    ),
    test_suite="tests",
    url="https://github.com/disco-org/disco",
    version=version,
    zip_safe=False,
)
