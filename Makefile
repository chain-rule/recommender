URL := http://files.grouplens.org/datasets/movielens/ml-1m.zip

test: tests/fixtures
	cargo test -vv

tests/fixtures:
	mkdir -p $@
	curl -L ${URL} -o $@/data.zip
	cd $@ && unzip -j data.zip

.PHONY: test
