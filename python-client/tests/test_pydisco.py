#!/usr/bin/env python
# -*- coding: utf-8 -*-

import typing

import pytest
from click.testing import CliRunner

from discodisco import cli, disco


@pytest.fixture
def response() -> None:
    """Sample pytest fixture.

    See more at: http://doc.pytest.org/en/latest/fixture.html
    """
    pass


@pytest.mark.parametrize(
    "input_obj,output_obj",
    [("a", "a")],
)
def test_parameters(input_obj: str, output_obj: str) -> None:
    assert input_obj == output_obj


def test_content(response: typing.Any) -> None:
    """Sample pytest test function with the pytest fixture as an argument."""
    # from bs4 import BeautifulSoup
    # assert 'GitHub' in BeautifulSoup(response.content).title.string
    pass


def test_cli_start() -> None:
    """Test the CLI."""
    runner = CliRunner()
    result = runner.invoke(cli.main)
    assert result.exit_code == 0
    assert "Server" in result.output


def test_cli_help() -> None:
    """Test the CLI."""
    runner = CliRunner()
    help_result = runner.invoke(cli.main, ["--help"])
    assert help_result.exit_code == 0
    assert "--help  Show this message and exit." in help_result.output
