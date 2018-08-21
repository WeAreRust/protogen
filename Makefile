cargo=cargo
protoc=protoc
plugin_path=target/debug/protogen
out_dir=./

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
	rm -f protos/**/*.ast.txt
	rm -f protos/**/*.proto.ts

.PHONY: build generate clean
