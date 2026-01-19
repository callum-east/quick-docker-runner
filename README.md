# quick-docker-runner



## Getting started

Download binary file, give it permissions to execute, and move to /usr/bin/ or ~/.local/bin


```
chmod +x quick-docker-runner
mv quick-docker-runner ~/.local/bin/
quick-docker-runner
```

##Building from source

Clone repository and build like any other Rust project

```
git clone https://github.com/callum-east/quick-docker-runner.git
cd quick-docker-runner
cargo run

```

For release from source run 

```
cargo run --release
cd target/release
chmod +x quick-docker-runner
./quick-docker-runner
```
