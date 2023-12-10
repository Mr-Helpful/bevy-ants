# Pheremones

## Requirements

Pheremones within a simulation:

- [ ] are deposited by Ants
- [ ] are spread over an area
- [ ] fade over time
- [ ] can be read by Ants

## Implementation

I'm going to be using a 2 camera approach to implement Pheremone placing.

### Overview

#### Components

- A secondary camera
- A "background" image
- Pheremones for each ant `[x] are spread over an area`

#### Algorithm

1. The pheremones are added as children of the Ants and hence follow their position. `[x] are deposited by Ants`
2. The secondary camera renders the background image and the pheremones and then blurs and dims the image. `[x] fade over time`
3. The secondary camera renders this to the background image
4. Only the background image is rendered by the main camera
5. Ants read the background image for pheremone intensity `[x] can be read by Ants`

#### Details

- [x] The secondary camera uses a compute shader for blurring and dimming.
  - [x] Need to learn a bit about compute shaders in bevy for this one, see:
    - <https://bevyengine.org/examples/Shaders/animate-shader/>
    - <https://github.com/alphastrata/shadplay/>
  - [x] Need to create a shader that blur an input texture
  - [x] Need to work out how to create a shader for an image handle, see:
    - <https://bevyengine.org/examples/Shaders/shader-material/>
    - <https://github.com/bevyengine/bevy/issues/3674>
    - <https://docs.rs/bevy/latest/bevy/sprite/trait.Material2d.html>
- [x] The rendering of the pheremones + background is controlled with render layers
  - <https://bevy-cheatbook.github.io/graphics/camera.html#render-layers>
- [x] Rendering to the background is acheived by rendering to its texture
  - <https://github.com/bevyengine/bevy/discussions/9036>
  - <https://bevyengine.org/examples/3D%20Rendering/render-to-texture/>
