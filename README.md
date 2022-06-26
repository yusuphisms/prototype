# prototype
Learning Rust &lt;> Proto

## Installation
Downloading was kinda weird. Go to the [Github releases page.](https://github.com/protocolbuffers/protobuf/releases)

Find release you want. Find version of zip you want, copy the URL. Something like: https://github.com/protocolbuffers/protobuf/releases/download/v21.2/protoc-21.2-linux-x86_64.zip

Run: `curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v21.2/protoc-21.2-linux-x86_64.zip`

Unzip: `unzip unzip protoc-21.2-linux-x86_64.zip`. Binary will be in a `bin` directory where you ran the command. Can move it where you want.

`mv ./bin/protoc /usr/local/bin/protoc`

`protoc --version`
