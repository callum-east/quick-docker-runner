# quick-docker-runner



## Getting started

Download binary file, give it permissions to execute, and move to /usr/bin/ or ~/.local/bin


```
chmod +x docker_runner
mv docker_runner ~/.local/bin/
docker_runner
```

## Building from source

Clone repository and build like any other Rust project

```
git clone https://github.com/callum-east/quick-docker-runner.git
cd quick-docker-runner
cargo run
```

For release from source run 

```
cargo build --release
cd target/release
chmod +x docker_runner
./docker_runner
```
