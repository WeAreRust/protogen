
Build protos with:
```sh
protoc --plugin=protoc-gen-custom=./target/debug/protogen \
       --custom_out=./protos \
       protos/test.proto
```
