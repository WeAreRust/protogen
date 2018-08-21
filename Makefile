cargo=cargo
protoc=protoc
plugin_path=target/debug/protogen
out_dir=protos

all: build generate

build:
	$(cargo) build

generate:
	$(protoc) \
		--plugin=protoc-gen-custom=$(plugin_path) \
		--custom_out=$(out_dir) \
		protos/**/*.proto

clean:
	rm -f output.txt

.PHONY: build generate clean
