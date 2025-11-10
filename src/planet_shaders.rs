// planet_shaders.rs
// Procedural planet shaders using mathematical functions

use raylib::prelude::*;
use crate::fragment::Fragment;
use crate::Uniforms;
use crate::noise::*;
use std::f32::consts::PI;

/// Shader type selection
#[derive(Clone, Copy, PartialEq)]
pub enum ShaderType {
    Rocky,          // Rocky planet with craters
    GasGiant,       // Gas giant with bands
    Lava,           // Lava planet (sci-fi)
    IceWorld,       // Frozen planet
    CloudPlanet,    // Planet with atmosphere
    Rings,          // Planet with rings
}

/// Mix two colors with a factor
#[inline]
fn mix_color(a: Vector3, b: Vector3, t: f32) -> Vector3 {
    let t = t.clamp(0.0, 1.0);
    Vector3::new(
        a.x * (1.0 - t) + b.x * t,
        a.y * (1.0 - t) + b.y * t,
        a.z * (1.0 - t) + b.z * t,
    )
}

/// Smooth step function
#[inline]
fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Rocky planet shader - Mars-like with craters and terrain
pub fn rocky_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let base_color = fragment.color;
    let time = uniforms.time * 0.1;

    // Base terrain using fBm
    let terrain = fbm(Vector3::new(pos.x * 3.0, pos.y * 3.0, pos.z * 3.0), 6, 2.0, 0.5);

    // Craters using voronoi
    let craters = voronoi(pos, 4.0);
    let crater_mask = smoothstep(0.25, 0.45, craters); // transición más suave

    // Softer rocky palette
    let dark_rock = Vector3::new(0.25, 0.15, 0.10);
    let mid_rock = Vector3::new(0.55, 0.35, 0.22);
    let light_rock = Vector3::new(0.85, 0.65, 0.45);
    let rust_tint = Vector3::new(0.8, 0.3, 0.15);

    // Mix colors smoothly
    let mut color = mix_color(dark_rock, mid_rock, terrain);
    color = mix_color(color, light_rock, terrain.powf(1.4));
    color = mix_color(color, rust_tint, (terrain * 1.3).sin().abs() * 0.25);

    // Make craters less deep (less black)
    color = mix_color(color, dark_rock * 0.9, 1.0 - crater_mask);

    // Clamp to avoid “burnt” black spots (set a minimum brightness)
    let min_brightness = 0.08; // evita tonos casi negros
    color = Vector3::new(
        color.x.max(min_brightness),
        color.y.max(min_brightness),
        color.z.max(min_brightness),
    );

    // Apply lighting and preserve intensity
    let lit_color = color * base_color;
    let original_intensity = (base_color.x + base_color.y + base_color.z) / 3.0;
    let mixed_intensity = (lit_color.x + lit_color.y + lit_color.z) / 3.0;

    if mixed_intensity > 0.001 {
        lit_color * (original_intensity / mixed_intensity)
    } else {
        lit_color
    }
}


/// Gas giant shader - Jupiter-like with turbulent bands
pub fn gas_giant_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let base_color = fragment.color;
    let time = uniforms.time * 0.05;

    // Latitude-based bands (horizontal)
    let latitude = pos.y;
    let band_pattern = ((latitude * 10.0 + time).sin() * 0.5 + 0.5).powf(1.2);

    // Add turbulence and swirl for motion
    let turb = turbulence(Vector3::new(pos.x * 2.0 + time, pos.y * 4.0, pos.z * 2.0), 5);
    let swirl = warp_noise(Vector3::new(pos.x + time * 0.5, pos.y * 2.0, pos.z), 0.6);

    // Jupiter-like palette (beige, orange, brown, cream)
    let light_band = Vector3::new(0.95, 0.85, 0.7);
    let mid_band = Vector3::new(0.85, 0.55, 0.35);
    let dark_band = Vector3::new(0.55, 0.35, 0.25);
    let storm_color = Vector3::new(1.0, 0.9, 0.85);

    // Combine bands and turbulence
    let mut color = mix_color(light_band, mid_band, band_pattern);
    color = mix_color(color, dark_band, turb * 0.4);

    // Add swirling storms and oval zones
    let storm_mask = smoothstep(0.65, 0.8, swirl);
    color = mix_color(color, storm_color, storm_mask * 0.7);

    // Apply lighting and normalization
    let lit_color = color * base_color;
    let original_intensity = (base_color.x + base_color.y + base_color.z) / 3.0;
    let mixed_intensity = (lit_color.x + lit_color.y + lit_color.z) / 3.0;

    if mixed_intensity > 0.001 {
        lit_color * (original_intensity / mixed_intensity)
    } else {
        lit_color
    }
}


/// Lava planet shader - Sci-fi molten world
pub fn lava_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let base_color = fragment.color;
    let time = uniforms.time * 0.3;
    
    // Flowing lava using warped noise
    let lava_flow = warp_noise(
        Vector3::new(pos.x * 2.0, pos.y * 2.0 + time, pos.z * 2.0),
        0.8
    );
    
    // Cracks using ridged noise
    let cracks = ridged_noise(
        Vector3::new(pos.x * 5.0, pos.y * 5.0, pos.z * 5.0),
        4
    );
    
    // Pulsing effect
    let pulse = ((time * 2.0).sin() * 0.5 + 0.5) * 0.3;
    
    // Color palette - blacks, reds, oranges, yellows
    let dark_crust = Vector3::new(0.1, 0.05, 0.0);
    let hot_lava = Vector3::new(1.0, 0.3, 0.0);
    let bright_lava = Vector3::new(1.0, 0.8, 0.1);
    
    // Mix based on flow pattern
    let mut color = mix_color(dark_crust, hot_lava, lava_flow);
    
    // Add bright cracks
    let crack_mask = smoothstep(0.6, 0.7, cracks);
    color = mix_color(color, bright_lava, crack_mask);
    
    // Add pulsing glow
    color = color + Vector3::new(pulse, pulse * 0.3, 0.0);
    
    // Apply lighting (less effect on emissive materials)
    let lit_color = Vector3::new(
        color.x * (base_color.x * 0.5 + 0.5),
        color.y * (base_color.y * 0.5 + 0.5),
        color.z * (base_color.z * 0.5 + 0.5),
    );
    
    lit_color
}

/// Ice world shader - Frozen planet with ice patterns
pub fn ice_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let base_color = fragment.color;
    let time = uniforms.time * 0.02;

    // Structural patterns
    let crystals = voronoi(pos, 6.0);
    let snow = fbm(Vector3::new(pos.x * 8.0, pos.y * 8.0, pos.z * 8.0), 5, 2.0, 0.5);
    let frost = value_noise(Vector3::new(pos.x * 15.0 + time, pos.y * 15.0, pos.z * 15.0));

    // Enhanced ice palette
    let deep_ice = Vector3::new(0.3, 0.6, 0.9);  // cold blue
    let surface_ice = Vector3::new(0.8, 0.9, 1.0); // pale ice
    let bright_snow = Vector3::new(1.0, 1.0, 1.0); // pure snow

    // Mix ice and snow
    let mut color = mix_color(deep_ice, surface_ice, snow);
    
    // Highlight crystalline regions
    let sparkle_mask = smoothstep(0.75, 0.85, crystals);
    color = mix_color(color, bright_snow, sparkle_mask * frost * 1.2);

    // Lighting with cold tint
    let lit_color = color * (base_color + Vector3::new(0.2, 0.25, 0.3));
    let original_intensity = (base_color.x + base_color.y + base_color.z) / 3.0 + 0.2;
    let mixed_intensity = (lit_color.x + lit_color.y + lit_color.z) / 3.0;

    if mixed_intensity > 0.001 {
        lit_color * (original_intensity / mixed_intensity)
    } else {
        lit_color
    }
}

/// Cloud planet shader - Earth-like with procedural clouds
pub fn cloud_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos = fragment.world_position;
    let base_color = fragment.color;
    let time = uniforms.time * 0.1;
    
    // Ocean vs land
    let land_mask = fbm(
        Vector3::new(pos.x * 2.0, pos.y * 2.0, pos.z * 2.0),
        6,
        2.0,
        0.5
    );
    
    // Cloud layer
    let clouds = fbm(
        Vector3::new(pos.x * 4.0 + time, pos.y * 4.0, pos.z * 4.0),
        4,
        2.0,
        0.6
    );
    
    // Vegetation detail
    let vegetation = value_noise(
        Vector3::new(pos.x * 10.0, pos.y * 10.0, pos.z * 10.0)
    );
    
    // Color palette
    let ocean = Vector3::new(0.1, 0.3, 0.6);
    let land = Vector3::new(0.4, 0.5, 0.3);
    let forest = Vector3::new(0.2, 0.4, 0.2);
    let cloud_color = Vector3::new(1.0, 1.0, 1.0);
    
    // Mix surface colors
    let surface_threshold = 0.45;
    let mut color = if land_mask > surface_threshold {
        mix_color(land, forest, vegetation)
    } else {
        ocean
    };
    
    // Add clouds
    let cloud_mask = smoothstep(0.5, 0.6, clouds);
    color = mix_color(color, cloud_color, cloud_mask * 0.7);
    
    // Apply lighting
    let lit_color = Vector3::new(
        color.x * base_color.x,
        color.y * base_color.y,
        color.z * base_color.z,
    );
    
    let original_intensity = (base_color.x + base_color.y + base_color.z) / 3.0;
    let mixed_intensity = (lit_color.x + lit_color.y + lit_color.z) / 3.0;
    
    if mixed_intensity > 0.001 {
        lit_color * (original_intensity / mixed_intensity)
    } else {
        lit_color
    }
}

/// Main shader dispatcher
pub fn apply_planet_shader(
    fragment: &Fragment,
    uniforms: &Uniforms,
    shader_type: ShaderType,
) -> Vector3 {
    match shader_type {
        ShaderType::Rocky => rocky_shader(fragment, uniforms),
        ShaderType::GasGiant => gas_giant_shader(fragment, uniforms),
        ShaderType::Lava => lava_shader(fragment, uniforms),
        ShaderType::IceWorld => ice_shader(fragment, uniforms),
        ShaderType::CloudPlanet => cloud_planet_shader(fragment, uniforms),
        ShaderType::Rings => rocky_shader(fragment, uniforms), // Base for ring planet
    }
}