// SPDX-FileCopyrightText: 2023 Tatsuhiro Ikebe <beike315@icloud.com>
// SPDX-License-Identifier: Apache-2.0

use nav2_send_goal_rust::Waypoint;

use r2r::{nav2_msgs::action::NavigateToPose, ClockType::RosTime};

use async_std::task;
use colored::Colorize;
use futures::StreamExt;
use std::sync::{Arc, Mutex};

pub async fn action_client(
    arc_node: Arc<Mutex<r2r::Node>>,
    waypoint: Waypoint,
    action_server_name: &String,
) -> Result<(), r2r::Error> {
    let (client, service_available) = {
        let mut node = arc_node.lock().unwrap();
        let client = node.create_action_client::<NavigateToPose::Action>(action_server_name)?;
        let service_available = node.is_available(&client)?;
        (client, service_available)
    };
    println!("{}", "waiting for service...".yellow());
    service_available.await?;
    println!("{}", "service available.".green());

    let goal_pose = set_goal(waypoint);

    let (goal, result, feedback) = client
        .send_goal_request(goal_pose)
        .expect("")
        .await
        .expect("Goal Rejected");

    task::spawn(feedback.for_each(move |msg| {
        let goal = goal.clone();
        async move {
            println!(
                "got feedback msg [ Distance Remaining: {:.3} -- {:?} ]",
                msg.distance_remaining,
                goal.get_status()
            );
        }
    }));

    match result.await {
        Ok((status, _msg)) => {
            println!("got action result {} ", status.to_string().green());
            std::process::exit(0);
        }
        Err(e) => println!("action failed: {:?}", e.to_string().red()),
    }

    Ok(())
}

fn set_goal(waypoint: Waypoint) -> NavigateToPose::Goal {
    let mut goal_pose = NavigateToPose::Goal::default();

    r2r::Clock::create(r2r::ClockType::SystemTime).unwrap();
    let clock = r2r::Clock::create(RosTime);
    let now = clock.unwrap().get_now();

    let mut header = r2r::std_msgs::msg::Header::default();
    header.frame_id = "map".to_string();
    header.stamp = r2r::Clock::to_builtin_time(&now.unwrap());

    let mut pose = r2r::geometry_msgs::msg::Pose::default();

    pose.position.x = waypoint.get_x();
    pose.position.y = waypoint.get_y();
    pose.orientation.w = waypoint.get_quaternion_w();
    pose.orientation.z = waypoint.get_quaternion_z();

    goal_pose.pose.header = header;
    goal_pose.pose.pose = pose;

    goal_pose
}
