use crate::observation::components::celestial_body::CelestialBody;
use bevy::mesh::Mesh3d;
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{
    Assets, Bundle, Color, Handle, LinearRgba, Mesh, ResMut, StandardMaterial, Transform, Vec3,
};

const REFERENCE_LUMINANCE: f32 = 500.0;
const REFERENCE_MAGNITUDE: f32 = 0.0;
const MIN_LUMINANCE: f32 = 0.0001;
const MAX_LUMINANCE: f32 = 100000.0;
const POGSON_RATIO: f32 = 2.512_f32;

#[derive(Bundle)]
pub struct SkyObject {
    body: CelestialBody,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
}

impl SkyObject {
    pub fn new(
        body: CelestialBody,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        let scale = if body.get().is_sun() {
            50.0
        } else if body.get().is_planet() {
            4.0
        } else if body.get().is_moon() {
            20.0
        } else {
            2.0
        };

        let luminance = magnitude_to_luminance(body.get().visual_magnitude() as f32);
        let mesh_handle = meshes.add(bevy::prelude::Sphere::new(1.0));
        let material_handle = create_new_material(Color::WHITE, luminance, materials);

        Self {
            body,
            mesh: Mesh3d(mesh_handle),
            material: MeshMaterial3d(material_handle),
            transform: Transform::default().with_scale(Vec3::splat(scale)),
        }
    }
}

fn create_new_material(
    color: Color,
    luminance: f32,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    let rgba = color.to_linear();
    materials.add(StandardMaterial {
        base_color: color,
        emissive: LinearRgba::rgb(
            rgba.red * luminance,
            rgba.green * luminance,
            rgba.blue * luminance,
        ),
        metallic: 0.0,
        reflectance: 0.0,
        ..Default::default()
    })
}

fn magnitude_to_luminance(magnitude: f32) -> f32 {
    let luminance = REFERENCE_LUMINANCE * POGSON_RATIO.powf(REFERENCE_MAGNITUDE - magnitude);
    luminance.clamp(MIN_LUMINANCE, MAX_LUMINANCE)
}
