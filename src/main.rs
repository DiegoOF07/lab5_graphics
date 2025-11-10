// main.rs
// Main rendering loop with planet shader system

mod framebuffer;
mod triangle;
mod obj;
mod matrix;
mod fragment;
mod vertex;
mod camera;
mod shaders;
mod light;
mod noise;
mod planet_shaders;
mod ring_shader;

use triangle::triangle;
use obj::Obj;
use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::f32::consts::PI;
use matrix::{create_model_matrix, create_projection_matrix, create_viewport_matrix};
use vertex::Vertex;
use camera::Camera;
use shaders::vertex_shader;
use light::Light;
use planet_shaders::{apply_planet_shader, ShaderType};

/// Uniforms structure containing transformation matrices and time
pub struct Uniforms {
    pub model_matrix: Matrix,
    pub view_matrix: Matrix,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
    pub time: f32,
}

/// Current rendering mode
struct RenderState {
    shader_type: ShaderType,
    show_rings: bool,
}

/// Main rendering pipeline
fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    light: &Light,
    state: &RenderState,
) {
    // Stage 1: Vertex Shader - Transform all vertices
    let transformed_vertices: Vec<Vertex> = vertex_array
        .iter()
        .map(|vertex| vertex_shader(vertex, uniforms))
        .collect();

    // Stage 2: Primitive Assembly - Group vertices into triangles
    let triangles: Vec<[Vertex; 3]> = transformed_vertices
        .chunks_exact(3)
        .map(|chunk| [chunk[0].clone(), chunk[1].clone(), chunk[2].clone()])
        .collect();

    // Stage 3: Rasterization - Convert triangles to fragments
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], light));
    }

    // Stage 4: Fragment Processing - Apply planet shader
    for fragment in fragments {
        let final_color = apply_planet_shader(&fragment, uniforms, state.shader_type);
            
        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            fragment.depth,
            final_color,
        );
    }

    // Optional: Render rings if enabled
    if state.show_rings {
        render_rings(framebuffer, uniforms, vertex_array, light);
    }
}

/// Renders rings around the planet
fn render_rings(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    light: &Light,
) {
    // Filter and transform vertices for ring geometry
    let ring_vertices: Vec<Vertex> = vertex_array
        .iter()
        .filter_map(|vertex| ring_shader::ring_vertex_shader(vertex, uniforms))
        .map(|vertex| vertex_shader(&vertex, uniforms))
        .collect();

    if ring_vertices.is_empty() {
        return;
    }

    // Assemble triangles
    let triangles: Vec<[Vertex; 3]> = ring_vertices
        .chunks_exact(3)
        .map(|chunk| [chunk[0].clone(), chunk[1].clone(), chunk[2].clone()])
        .collect();

    // Rasterize
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], light));
    }

    // Apply ring shader
    for fragment in fragments {
        let final_color = ring_shader::ring_fragment_shader(&fragment, uniforms);
            
        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            fragment.depth,
            final_color,
        );
    }
}

fn main() {
    const WINDOW_WIDTH: i32 = 1300;
    const WINDOW_HEIGHT: i32 = 900;

    let (mut window, raylib_thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Procedural Planet Renderer")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    
    // Initialize camera
    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 5.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    // Light source
    let light = Light::new(Vector3::new(5.0, 5.0, 5.0));

    // Model transformation parameters
    let translation = Vector3::new(0.0, 0.0, 0.0);
    let scale = 1.0;
    let mut rotation = Vector3::new(0.0, 0.0, 0.0);

    // Load sphere model
    let obj = Obj::load("./models/sphere.obj").expect("Failed to load sphere model");
    let vertex_array = obj.get_vertex_array();

    // Render state
    let mut state = RenderState {
        shader_type: ShaderType::Rocky,
        show_rings: false,
    };

    framebuffer.set_background_color(Color::new(10, 10, 20, 255));

    println!("\n=== PLANET SHADER CONTROLS ===");
    println!("1 - Rocky Planet (Mars-like)");
    println!("2 - Gas Giant (Jupiter-like)");
    println!("3 - Lava Planet (Sci-fi)");
    println!("4 - Ice World");
    println!("5 - Cloud Planet (Earth-like)");
    println!("R - Toggle Rings");
    println!("SPACE - Auto-rotate");
    println!("==============================\n");

    let mut auto_rotate = false;

    // Main render loop
    while !window.window_should_close() {
        // Handle shader switching
        if window.is_key_pressed(KeyboardKey::KEY_ONE) {
            state.shader_type = ShaderType::Rocky;
            state.show_rings = false;
            println!("Selected: Rocky Planet");
        }
        if window.is_key_pressed(KeyboardKey::KEY_TWO) {
            state.shader_type = ShaderType::GasGiant;
            state.show_rings = false;
            println!("Selected: Gas Giant");
        }
        if window.is_key_pressed(KeyboardKey::KEY_THREE) {
            state.shader_type = ShaderType::Lava;
            state.show_rings = false;
            println!("Selected: Lava Planet");
        }
        if window.is_key_pressed(KeyboardKey::KEY_FOUR) {
            state.shader_type = ShaderType::IceWorld;
            state.show_rings = false;
            println!("Selected: Ice World");
        }
        if window.is_key_pressed(KeyboardKey::KEY_FIVE) {
            state.shader_type = ShaderType::CloudPlanet;
            state.show_rings = false;
            println!("Selected: Cloud Planet");
        }
        if window.is_key_pressed(KeyboardKey::KEY_R) {
            state.show_rings = !state.show_rings;
            println!("Rings: {}", if state.show_rings { "ON" } else { "OFF" });
        }
        if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
            auto_rotate = !auto_rotate;
            println!("Auto-rotate: {}", if auto_rotate { "ON" } else { "OFF" });
        }

        // Auto-rotation
        if auto_rotate {
            rotation.y += 0.01;
        }

        camera.process_input(&window);
        framebuffer.clear();
        
        // Create transformation matrices
        let model_matrix = create_model_matrix(translation, scale, rotation);
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(
            PI / 3.0,
            WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
            0.1,
            100.0
        );
        let viewport_matrix = create_viewport_matrix(
            0.0,
            0.0,
            WINDOW_WIDTH as f32,
            WINDOW_HEIGHT as f32
        );

        // Package uniforms
        let uniforms = Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time: window.get_time() as f32,
        };

        render(&mut framebuffer, &uniforms, &vertex_array, &light, &state);
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}