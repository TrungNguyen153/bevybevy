use avian2d::prelude::*;
use bevy::prelude::*;

use super::Player;

// Contract format
// Contacts {
//     entity1: 19v1#4294967315,
//     entity2: 21v1#4294967317,
//     body_entity1: Some(
//         19v1#4294967315,
//     ),
//     body_entity2: Some(
//         21v1#4294967317,
//     ),
//     manifolds: [
//         ContactManifold {
//             contacts: [
//                 ContactData {
//                     point1: Vec2(
//                         -89.7749,
//                         5.0,
//                     ),
//                     point2: Vec2(
//                         0.0,
//                         -8.0,
//                     ),
//                     normal1: Vec2(
//                         0.0,
//                         1.0,
//                     ),
//                     normal2: Vec2(
//                         0.0,
//                         -1.0,
//                     ),
//                     penetration: 0.00025177002,
//                     normal_impulse: 0.0,
//                     tangent_impulse: 0.0,
//                     feature_id1: PackedFeatureId(
//                         3221225473,
//                     ),
//                     feature_id2: PackedFeatureId(
//                         3221225472,
//                     ),
//                 },
//             ],
//             normal1: Vec2(
//                 0.0,
//                 1.0,
//             ),
//             normal2: Vec2(
//                 0.0,
//                 -1.0,
//             ),
//             index: 0,
//         },
//     ],
//     is_sensor: false,
//     during_current_frame: true,
//     during_previous_frame: true,
//     total_normal_impulse: 0.0,
//     total_tangent_impulse: 0.0,
// }

#[allow(clippy::type_complexity)]
pub fn kinematic_player_controller(
    collisions: Res<Collisions>,
    bodies: Query<&RigidBody>,
    collider_parents: Query<&ColliderParent, Without<Sensor>>,
    mut player: Query<(&mut Position, &mut LinearVelocity), (With<RigidBody>, With<Player>)>,
    time: Res<Time>,
) {
    // for contacts in collisions.iter() {
    //     println!("{contacts:#?}");
    //     // Get the rigid body entities of the colliders (colliders could be children)
    //     let Ok([collider_parent1, collider_parent2]) =
    //         collider_parents.get_many([contacts.entity1, contacts.entity2])
    //     else {
    //         continue;
    //     };

    //     // Get the body of the character controller and whether it is the first
    //     // or second entity in the collision.
    //     let is_first_collider: bool;

    //     let character_rb: RigidBody;
    //     let is_other_dynamic: bool;

    //     let (mut position, mut velocity) =
    //         if let Ok(player) = player.get_mut(collider_parent1.get()) {
    //             is_first_collider = true;
    //             character_rb = *bodies.get(collider_parent1.get()).unwrap();
    //             is_other_dynamic = bodies
    //                 .get(collider_parent2.get())
    //                 .is_ok_and(|c| c.is_dynamic());
    //             player
    //         } else if let Ok(player) = player.get_mut(collider_parent2.get()) {
    //             is_first_collider = false;
    //             character_rb = *bodies.get(collider_parent2.get()).unwrap();
    //             is_other_dynamic = bodies
    //                 .get(collider_parent1.get())
    //                 .is_ok_and(|c| c.is_dynamic());
    //             player
    //         } else {
    //             // unknown
    //             continue;
    //         };

    //     // This system only handles collision response for kinematic character controllers.
    //     if !character_rb.is_kinematic() {
    //         continue;
    //     }

    //     // Iterate through contact manifolds and their contacts.
    //     // Each contact in a single manifold shares the same contact normal.

    //     for manifold in contacts.manifolds.iter() {
    //         let normal = if is_first_collider {
    //             manifold.normal1
    //         } else {
    //             manifold.normal2
    //         };

    //         let mut deepest_penetration: Scalar = Scalar::MIN;

    //         // Solve each penetrating contact in the manifold.
    //         for contact in manifold.points.iter() {
    //             if contact.penetration > 0.0 {
    //                 position.0 += normal * contact.penetration;
    //             }
    //             deepest_penetration = deepest_penetration.max(contact.penetration);
    //         }
    //     }
    // }
}
