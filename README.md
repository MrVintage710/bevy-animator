
![bevy-animator-light](https://github.com/MrVintage710/bevy-animator/assets/8060151/276a32dc-a3a9-4944-8dca-103570b06794#gh-dark-mode-only)
![bevy-animator-dark](https://github.com/MrVintage710/bevy-animator/assets/8060151/08bf0dde-9de6-4d7f-bf90-0c46ce5eeccf#gh-light-mode-only)

** This Crate is still work in progress and isn't feature complete or stable. **

# Bevy Animator

Bevy animator is an animation system desgined for the bevy game engine. It is designed to remove a large amount of boilerplate required for animation, and allows for state based animations.

## Motivation

The State or animation in bevy varried. Bevy has an implemnetation for GLTF animations files, but no support for 2D animation. There are many 3rd party animation libraries that are great, but it can be difficult to controll the animations as well as use them together. This crate was made as a one stop shop for animations, while also giving and easy to use api for updating the animation state.

## Supported Animations

- [X] Aseprite (Since 0.1.0)
- [ ] BlockBench
- [ ] GLTF and GLB

.. More to come.

## How To



## Contributing

To contribute, you can open an issue along with a feature branch. When done with the feature, you can make a PR and one of the maintainers will edit and close the PR. Some things to look out for while making a feature:

- Each feature should only have one type of animation that it is looking to support.
- Each feature should contain all logic code within a aptly named module (for example, all aseprite code is located in 'aseprite.rs')
- All code should be documented so those that work on it later have a guide.
