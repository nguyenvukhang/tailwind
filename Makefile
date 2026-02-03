current: run

generate:
	python3 -m tw.lsp

run: generate
	python3 -m tw
