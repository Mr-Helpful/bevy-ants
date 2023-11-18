# Pheremones

## Overview

Pheremones within a simulation:

1. are deposited by Ants
2. are spread over an area
3. fade over time
4. can be read by Ants

## Implementations

### Double rendering

#### Overview

Components:

- A secondary camera
- A "background" image
- Pheremones for each ant

Algorithm:

1. The secondary camera renders the background image and the pheremones and then blurs and dims the image.
2. The secondary camera renders this to the background image
3. Only the background image is rendered by the main camera

#### Details

- The secondary camera uses a compute shader for blurring and dimming.
- The rendering of the pheremones + background is controlled with [Render Layers](https://bevy-cheatbook.github.io/graphics/camera.html#render-layers)
