pub mod send_goal;

use nav2_send_goal_rust::Waypoint;

use colored::Colorize;
use std::sync::{Arc, Mutex};
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let action_server_name = "navigate_to_pose";

    let goal = Waypoint {
        x: 1.0,
        y: -0.5,
        yaw: 0.1,
    };

    println!(
        "{} [x:{}, y:{}, yaw:{}, w:{}, z:{}]",
        "Set Goal Point".green(),
        goal.get_x(),
        goal.get_y(),
        goal.get_yaw(),
        goal.get_quaternion_w(),
        goal.get_quaternion_z()
    );

    let ctx = r2r::Context::create()?;
    let node = r2r::Node::create(ctx, "nav2_send_goal_rust", "")?;
    let arc_node = Arc::new(Mutex::new(node));
    let an = arc_node.clone();

    task::spawn(async move {
        send_goal::action_client(an, goal, &action_server_name.to_string())
            .await
            .unwrap()
    });

    let handle = tokio::task::spawn_blocking(move || loop {
        {
            arc_node
                .lock()
                .unwrap()
                .spin_once(std::time::Duration::from_millis(10));
        }
        std::thread::sleep(std::time::Duration::from_millis(100))
    });

    handle.await?;

    Ok(())
}
