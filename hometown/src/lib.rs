pub mod rain;

/// There are a couple of ways to deal with multiple dimensions... One would be to define our own
/// type inside of thomas and export it
struct Frustrum {
    x: f32,
    y: f32,
    z: f32
}

/// Another wuold be to simply use cgmath
struct Any {
    any: thomas::cgmath::Vector3<f32>
}

/// Finally, how does ggez do it?i
