# -*- coding: utf-8 -*-

"""Console script for discodisco."""
import sys
import typing

import click

from discodisco import Server, Parameterizer, Analyzer


@click.command()
def main(args: typing.Optional[str] = None) -> int:
    """Console script for discodisco."""
    # todo: read the env vars for ports of the server
    disco = Server()
    # disco.start()
    # disco.stop()
    # if not running, start it and block until ctrl c for graceful shutdown
    print(disco.__class__.__qualname__)
    return 0


if __name__ == "__main__":
    sys.exit(main())  # pragma: no cover
