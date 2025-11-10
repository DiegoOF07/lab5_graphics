// ring_shader.rs
// Procedural ring generation using vertex deformation

use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::noise::value_noise;

/// Vertex shader for creating rings around a planet
/// Deforms sphere geometry into a ring shape
pub fn ring_vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Option<Vertex> {
    let pos = vertex.position;
    
    // Calculate distance from Y axis (cylindrical radius)
    let radius = (pos.x * pos.x + pos.z * pos.z).sqrt();
    
    // Ring parameters
    let inner_radius = 1.5;
    let outer_radius = 2.5;
    let ring_thickness = 0.05;
    
    // Only render vertices within ring bounds
    if radius < inner_radius || radius > outer_radius {
        return None; // Discard vertex outside ring
    }
    
    // Flatten vertex to ring plane (Y = 0 with small variation)
    let noise_offset = value_noise(Vector3::new(
        pos.x * 10.0,
        0.0,
        pos.z * 10.0
    )) * ring_thickness;
    
    let ring_pos = Vector3::new(
        pos.x,
        noise_offset,
        pos.z
    );
    
    // Create modified vertex
    let mut ring_vertex = vertex.clone();
    ring_vertex.position = ring_pos;
    ring_vertex.normal = Vector3::new(0.0, 1.0, 0.0); // Point up
    
    Some(ring_vertex)
}

/// Fragment shader for ring coloring
pub fn ring_fragment_shader(
    fragment: &crate::fragment::Fragment,
    uniforms: &Uniforms,
) -> Vector3 {
    let pos = fragment.world_position;
    let base_color = fragment.color;
    
    // Calculate distance from center
    let radius = (pos.x * pos.x + pos.z * pos.z).sqrt();
    
    // Create bands in the rings
    let band_pattern = (radius * 20.0).sin() * 0.5 + 0.5;
    
    // Add noise for detail
    let noise = value_noise(Vector3::new(
        pos.x * 15.0,
        pos.y * 15.0,
        pos.z * 15.0
    ));
    
    // Color palette - ice and rock particles
    let dark_band = Vector3::new(0.3, 0.25, 0.2);
    let light_band = Vector3::new(0.8, 0.75, 0.7);
    
    let mut color = Vector3::new(
        dark_band.x * (1.0 - band_pattern) + light_band.x * band_pattern,
        dark_band.y * (1.0 - band_pattern) + light_band.y * band_pattern,
        dark_band.z * (1.0 - band_pattern) + light_band.z * band_pattern,
    );
    
    // Add particle detail
    color = color * (0.8 + noise * 0.2);
    
    // Apply lighting
    let lit_color = Vector3::new(
        color.x * (base_color.x + 0.3),
        color.y * (base_color.y + 0.3),
        color.z * (base_color.z + 0.3),
    );
    
    lit_color
}