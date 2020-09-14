.PHONY: clean build release

TAG := $(shell cat Cargo.toml | grep "version" | sed "s/version = //;s/\"//g")

clean:
	@cargo clean

build:
	@cargo build

release:
	@cargo publish
	@cargo clean
	@git tag -a $(TAG) -m "Release: Version $(TAG)"
	@git push origin --tags
