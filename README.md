# prototype
Learning Rust &lt;> Proto

## Installation
Downloading was kinda weird. Go to the [Github releases page.](https://github.com/protocolbuffers/protobuf/releases)

Find release you want. Find version of zip you want, copy the URL. Something like: https://github.com/protocolbuffers/protobuf/releases/download/v21.2/protoc-21.2-linux-x86_64.zip

Run: `curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v21.2/protoc-21.2-linux-x86_64.zip`

Unzip: `unzip unzip protoc-21.2-linux-x86_64.zip`. Binary will be in a `bin` directory where you ran the command. Can move it where you want.

`mv ./bin/protoc /usr/local/bin/protoc`

`protoc --version`

## Getting Started

Follow these instructions to make first message in Proto3 https://developers.google.com/protocol-buffers/docs/proto3
Then follow prost_build for the rest


## What I Like / What I'm Meh About
Not seeing a whole lot of benefit of this over creating Rust structs with `serde` annotations.
It could just be my VS Code setup bugging out, but VS Code wasn't able to work well with rust-analyzer either? But I think my setup got messed up somehow.
I do like the idea of creating message schemas that I can use in other contexts as long as it has a way to compile protocol buffers.
