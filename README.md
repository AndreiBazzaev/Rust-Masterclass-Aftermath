## Description: Software Rasterizer with a camera, basic Gltf loading and shading made with the help of Rust!
---

# Title
Name: Andrei Bazzaev 

BUas email: 210632@buas.nl

## Features

+ Texturing - textures are depicted correctly on polygons
+ Model loading - simple models can be loaded - base color textures are applied
+ Direct shading - basic shading with a single direct light source
+ Scene - we can create and render multiple models, which can move, rotate, scale in the world
+ Culling - multiple culling algorithms are used for the performance boost (including backface  culling)
+ Camera - camera can move in the world with a mouse and keyboard
+ Zig-Zag Traversal - algorithm reduces the number of passes to the "pixel shader" 

* Use WASD + a mouse with left click to move and rotate the camera