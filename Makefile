URL := http://files.grouplens.org/datasets/movielens/ml-1m.zip

all: test

bench: tests/fixtures
	cargo bench -vv

test: tests/fixtures
	cargo test -vv

tests/fixtures:
	mkdir -p $@
	curl -L ${URL} -o $@/data.zip
	cd $@ && unzip -j data.zip

.PHONY: all bench test
