# list available commands
default:
  just --list

# install python dependencies
py-setup:
  cd py-stk && uv sync --all-extras

# build python docs
py-docs:
  cd py-stk && maturin dev
  cd py-stk && rm -rf docs/build docs/source/_autosummary
  cd py-stk && uv run make -C docs html
  echo Docs are in $PWD/py-stk/docs/build/html/index.html
