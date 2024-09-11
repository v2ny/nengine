# Nengine

**Nengine** is a modern 3D game engine built in Rust, focusing on performance, flexibility, and future scalability. This engine is designed to support advanced rendering, physics, and game mechanics, with a clean, efficient codebase aimed at both beginners and experienced developers.

## Key Features

- **Rendering API**: Initial focus on **OpenGL** with plans to integrate **Vulkan** in the future.
- **Physics Engine**: Powered by **Physsol** for fast and accurate 2D/3D physics.
- **Math Library**: Uses **nalgebra** for advanced linear algebra and 3D math operations.
- **Entity Component System (ECS)**: Built on **hecs** for efficient entity and data management.
- **Model Loader**: Custom Implemented to support OBJ, FBX, USDZ, GLB, GLTF, DAE and STL.
- **GUI Framework**: Integrated **egui** for immediate-mode graphical user interfaces.
- **Cross-Platform Support**: Initial support planned for **Windows**, **Linux**, and **macOS**.

## Roadmap & Feature Checklist

- [ ] OpenGL rendering pipeline
- [ ] Basic GUI (egui)
- [ ] Scripting integration (e.g., Lua, Typescript (Maybe, C++))
- [ ] Basic OBJ/GLTF model loading (From textures and materials to lighting, Advanced lighting.)
- [ ] Entity Component System integration (using HECS crate)
- [ ] Physics engine integration (Physsol)
- [ ] Vulkan rendering support
- [ ] Scene management system
- [ ] Skeletal animation system
- [ ] Particle system
- [ ] Sound engine
- [ ] Networking support
- [ ] Post Processing & VFX

## Supported Platforms

- **Windows**
- **Linux**
- **macOS**

## Contribution

This project is in its early stages, and contributions are welcome! Please submit any issues, suggestions, or pull requests to help improve the engine.

## License

Nengine is licensed under the **AGPL-3.0**. For more details, please look at the [LICENSE](LICENSE) file.

---

Nengine is currently in development stage, without any completed features or examples, and not recommended at all for uses other than educational purposes.
