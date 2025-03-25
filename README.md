## RP2040 base template

#### Getting started

##### Install prerequirements

```bash
rustup target install thumbv6m-none-eabi # add the target architecture for RP2040
cargo install flip-link # stack overflow protection
cargo install --locked elf2uf2-rs # create uf2 firmware files from elf binaries.
```

##### Get this template

```bash
cargo install cargo-generate # if you haven't already
cargo generate --git tovernator/rp2040-base
```

