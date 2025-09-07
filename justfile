# list available commands
default:
  just --list

# install python dependencies
py-setup:
  cd py-stk && uv sync --all-extras
