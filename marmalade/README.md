# Marmalade
![](resources/images/banner.png)

Marmalade is a small game library targeting the web.

It provides :
- A 2d rendering context with an intermediate abstraction level (somewhere between [macroquad](https://github.com/not-fl3/macroquad) and raw webgl)
- A simple way of loading and playing sounds
- Easy input handling

It will (probably) provide in the future :
- A 3d rendering context
- A WebGPU backend to replace WebGL once the API is stable

It won't :
- Support any other platform than HTML5 so that it can stay small and simple while being portable (It should however be possible to make a native app with the help of [Electron](https://www.electronjs.org/) or [Tauri](https://tauri.app))
- Hide rust specific complexity. You may need to use Rc, Cells and play with lifetimes for example

## How to use
Install [trunk](https://trunkrs.dev)
```bash
cargo install trunk
```

Run one of the [examples](examples)
```bash
git clone git@github.com:puzzle-rs/marmalade.git
cd marmalade/examples/hello-world
trunk serve
```
Then open your web browser on [localhost:8080](localhost:8080)

## License
The library itself is licensed under the [MPL-2.0 license](LICENSE)

The [Monogram font](https://datagoblin.itch.io/monogram) is included in the library and licensed under the [CC0 1.0 license](https://creativecommons.org/publicdomain/zero/1.0/)

The Marmalade logo can be used under the [CC BY 4.0 license](https://creativecommons.org/licenses/by/4.0/)