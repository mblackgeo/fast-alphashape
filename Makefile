DEFAULT_GOAL := help

help:
	@fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed -e 's/##//'

.PHONY: install
install:  ## Install dev requirements into the current python environment
	pip install -r requirements-dev.txt
	pre-commit install
	pre-commit install --hook-type commit-msg

.PHONY: build-dev
build-dev:  ## Build the package in debug and install it into the virtualenv
	maturin develop

.PHONY: build
build:  ## Build the optimised release binaries
	maturin build -r

.PHONY: lint
lint:  ## Run linting checks with cargo, flake8, isort, and black
	cargo fmt --check
	cargo clippy -- -D warnings
	flake8 python/
	black --check python/
	isort -c python/

.PHONY: test
test: build  ## Run the test suite using cargo and pytest
	cargo test
	pytest tests

