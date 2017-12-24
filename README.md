### Rust-Tracer
Basic ray tracer in Rust. Uses [cgmath](https://github.com/brendanzab/cgmath) for point/vector handling and [Piston's Image library](https://github.com/PistonDevelopers/image) to create the final image.

#### Examples 

All rendering done on a Raspberry Pi 3B with 4 threads

* Spheres and icosohedron - 800x800, 4x multisampling - 1m 36s

![Alt text](examples/Spheres-800x800-4x-96s.png?raw=true "Spheres")

* Cow - ~4,600 vertices - 1000x1000, 4x multisampling - 9m 53s

![Alt text](examples/Cow-1000x1000-4x-9m53s.png?raw=true "Cow")

* Mini Cooper - ~45,000 vertices - 1200x1200, 8x multisampling - 80m

![Alt text](examples/MINI-1200x1200-8x-80m.png?raw=true "MINI")


#### Supports:
- Basic shapes (triangle, rectangle, sphere) via .obj files
- Point lights
- Reflections and shadows
- Phong reflection model
- Reinhard tone mapping
- K-D trees
- Multithreaded rendering
- Multisampling
- Extensive JSON configuration

#### TODO:
- Refraction
- Textures
- etc.
