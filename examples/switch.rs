use bevy::{
    prelude::*,
    render::camera::{ActiveCameras, Camera, CameraPlugin},
};

use bevy_config_cam::next_enum;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// Used in queries when you want filter between cameras
#[derive(Clone, Eq, PartialEq, Debug, Hash, EnumIter, Component)]
enum SwitchableCameras {
    CubeCam,
    TopDownCam,
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_state(SwitchableCameras::CubeCam)
        .add_startup_system(setup)
        .add_system(cycle_camera_state)
        .add_system(switch_camera)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // cube camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(SwitchableCameras::CubeCam);

    // topdown camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            camera: Camera {
                name: Some("Inactive".to_string()),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 10.0, 0.1).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(SwitchableCameras::TopDownCam);
}

fn switch_camera(
    mut active_cams: ResMut<ActiveCameras>,
    cam_state: ResMut<State<SwitchableCameras>>,
    mut query: Query<(&SwitchableCameras, &mut Camera)>,
) {
    // remove current camera
    active_cams.remove(CameraPlugin::CAMERA_3D);

    // set all cameras to inactive
    for (_, mut bevy_cam) in query.iter_mut() {
        bevy_cam.name = Some("Inactive".to_string());
    }

    // find the camera with the current state, set its name to the 3d camera name
    for (_, mut bevy_cam) in query
        .iter_mut()
        .filter(|(switchable_cams, _)| cam_state.current().eq(switchable_cams))
    {
        bevy_cam.name = Some(CameraPlugin::CAMERA_3D.to_string());
    }

    // add the name of our active camera to the active cameras resource
    active_cams.add(CameraPlugin::CAMERA_3D);
}

fn cycle_camera_state(
    mut selected_cam: ResMut<State<SwitchableCameras>>,
    mut keys: ResMut<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::E) {
        let new_cam_state = next_enum!(SwitchableCameras, selected_cam);
        println!("New camera: {:?}", new_cam_state);
        selected_cam.set(new_cam_state).unwrap();

        keys.reset(KeyCode::E);
    }
}
