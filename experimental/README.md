## Experimental implementations

This directory contains python code for fast iteration and quick prototyping of ideas.

The goal is to test out new features and then port them over to the main rust backend when the interfaces have been finalized 

#### Installation

We use `pipenv` to manage the required python dependencies. Furthermore, `portaudio` has to be installed for `pyaudio` to work.

```bash
# on macOS, you can install portaudio with brew
sudo apt-get install -y portaudio19-dev 

pipenv install --dev
pipenv shell
```
