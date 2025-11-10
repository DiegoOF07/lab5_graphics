// noise.rs
// Procedural noise functions for shader effects

use raylib::math::Vector3;

/// Simple hash function for pseudo-random values
#[inline]
fn hash(p: Vector3) -> f32 {
    let mut h = p.x * 127.1 + p.y * 311.7 + p.z * 74.7;
    h = (h.sin() * 43758.5453).fract();
    h
}

/// 3D Value noise - simple but effective
pub fn value_noise(p: Vector3) -> f32 {
    let i = Vector3::new(p.x.floor(), p.y.floor(), p.z.floor());
    let f = Vector3::new(p.x.fract(), p.y.fract(), p.z.fract());
    
    // Smooth interpolation
    let u = Vector3::new(
        f.x * f.x * (3.0 - 2.0 * f.x),
        f.y * f.y * (3.0 - 2.0 * f.y),
        f.z * f.z * (3.0 - 2.0 * f.z),
    );

    // Sample corners of cube
    let a = hash(Vector3::new(i.x, i.y, i.z));
    let b = hash(Vector3::new(i.x + 1.0, i.y, i.z));
    let c = hash(Vector3::new(i.x, i.y + 1.0, i.z));
    let d = hash(Vector3::new(i.x + 1.0, i.y + 1.0, i.z));
    let e = hash(Vector3::new(i.x, i.y, i.z + 1.0));
    let f_val = hash(Vector3::new(i.x + 1.0, i.y, i.z + 1.0));
    let g = hash(Vector3::new(i.x, i.y + 1.0, i.z + 1.0));
    let h_val = hash(Vector3::new(i.x + 1.0, i.y + 1.0, i.z + 1.0));

    // Trilinear interpolation
    let k0 = a;
    let k1 = b - a;
    let k2 = c - a;
    let k3 = e - a;
    let k4 = a - b - c + d;
    let k5 = a - c - e + g;
    let k6 = a - b - e + f_val;
    let k7 = -a + b + c - d + e - f_val - g + h_val;

    k0 + k1 * u.x + k2 * u.y + k3 * u.z + k4 * u.x * u.y 
       + k5 * u.y * u.z + k6 * u.z * u.x + k7 * u.x * u.y * u.z
}

/// Fractal Brownian Motion (fBm) - layered noise for detail
pub fn fbm(p: Vector3, octaves: i32, lacunarity: f32, gain: f32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut pos = p;

    for _ in 0..octaves {
        value += amplitude * value_noise(Vector3::new(
            pos.x * frequency,
            pos.y * frequency,
            pos.z * frequency,
        ));
        frequency *= lacunarity;
        amplitude *= gain;
    }

    value
}

/// Turbulence - absolute values create sharp features
pub fn turbulence(p: Vector3, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut pos = p;

    for _ in 0..octaves {
        value += amplitude * value_noise(Vector3::new(
            pos.x * frequency,
            pos.y * frequency,
            pos.z * frequency,
        )).abs();
        frequency *= 2.0;
        amplitude *= 0.5;
    }

    value
}

/// Voronoi/cellular noise - creates cell-like patterns
pub fn voronoi(p: Vector3, scale: f32) -> f32 {
    let scaled = Vector3::new(p.x * scale, p.y * scale, p.z * scale);
    let cell = Vector3::new(scaled.x.floor(), scaled.y.floor(), scaled.z.floor());
    
    let mut min_dist = f32::INFINITY;

    // Check neighboring cells
    for z in -1..=1 {
        for y in -1..=1 {
            for x in -1..=1 {
                let neighbor = Vector3::new(
                    cell.x + x as f32,
                    cell.y + y as f32,
                    cell.z + z as f32,
                );
                
                let point = Vector3::new(
                    neighbor.x + hash(neighbor),
                    neighbor.y + hash(Vector3::new(neighbor.y, neighbor.x, neighbor.z)),
                    neighbor.z + hash(Vector3::new(neighbor.z, neighbor.y, neighbor.x)),
                );
                
                let diff = Vector3::new(
                    scaled.x - point.x,
                    scaled.y - point.y,
                    scaled.z - point.z,
                );
                
                let dist = (diff.x * diff.x + diff.y * diff.y + diff.z * diff.z).sqrt();
                min_dist = min_dist.min(dist);
            }
        }
    }

    min_dist
}

/// Ridged noise - inverted absolute noise for mountain ridges
pub fn ridged_noise(p: Vector3, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;

    for _ in 0..octaves {
        let n = value_noise(Vector3::new(
            p.x * frequency,
            p.y * frequency,
            p.z * frequency,
        ));
        value += (1.0 - n.abs()) * amplitude;
        frequency *= 2.0;
        amplitude *= 0.5;
    }

    value
}

/// Warp/domain distortion - creates swirling patterns
pub fn warp_noise(p: Vector3, amount: f32) -> f32 {
    let offset = Vector3::new(
        fbm(p, 4, 2.0, 0.5),
        fbm(Vector3::new(p.y, p.z, p.x), 4, 2.0, 0.5),
        fbm(Vector3::new(p.z, p.x, p.y), 4, 2.0, 0.5),
    );
    
    let warped = Vector3::new(
        p.x + offset.x * amount,
        p.y + offset.y * amount,
        p.z + offset.z * amount,
    );
    
    value_noise(warped)
}