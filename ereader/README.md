# gba game e-Reader loader

This is, as far as I can tell, the first ever e-Reader application written in rust
In order to build it and make it usable for the e-Reader you need the following packages:
- cargo/rustup
- just
- arm-none-eabi toolchain
- [CaitSith2's e-Reader tools](https://www.caitsith2.com/ereader/index.htm) [multi-platform port](https://github.com/breadbored/nedclib)
- [neflmake](https://github.com/AkBKukU/e-reader-dev) (only required if you want to inject the built vpk into a save)

## Building

In order to build run:

```bash
just build
```

If you want to create a vpk as well:

```bash
just vpk
```

If you want to create a save:

```bash
just save
```

If you need to clean run:

```bash
just clean
```

Keep in mind that this program is made with the US e-Reader in mind, not with the JP e-Reader+
