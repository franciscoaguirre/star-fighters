use bevy::prelude::*;

const BOUNDARY_DIMENSIONS: Vec2 = Vec2::new(725., 350.);

pub struct BoundariesPlugin;
impl Plugin for BoundariesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, wrap);
    }
}

fn wrap(mut query: Query<&mut Transform>) {
    for mut transform in query.iter_mut() {
        if transform.translation.x > BOUNDARY_DIMENSIONS.x {
            transform.translation.x = -BOUNDARY_DIMENSIONS.x;
        } else if transform.translation.x < -BOUNDARY_DIMENSIONS.x {
            transform.translation.x = BOUNDARY_DIMENSIONS.x;
        }

        if transform.translation.y > BOUNDARY_DIMENSIONS.y {
            transform.translation.y = -BOUNDARY_DIMENSIONS.y;
        } else if transform.translation.y < -BOUNDARY_DIMENSIONS.y {
            transform.translation.y = BOUNDARY_DIMENSIONS.y;
        }
    }
}
