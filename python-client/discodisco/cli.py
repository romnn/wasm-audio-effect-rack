# -*- coding: utf-8 -*-

"""Console script for discodisco."""
import sys
import typing

import click

from disco import *


@click.command()
def main(args: typing.Optional[str] = None) -> int:
    """Console script for discodisco."""
    # todo: read the env vars for ports of the server
    # if not running, start it and block until ctrl c for graceful shutdown
    print(sum_as_string(1, 2))
    return 0


if __name__ == "__main__":
    sys.exit(main())  # pragma: no cover
