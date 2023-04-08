# nav2_send_goal_rust [![build-test](https://github.com/uhobeike/nav2_send_goal_rust/actions/workflows/build-test.yaml/badge.svg)](https://github.com/uhobeike/nav2_send_goal_rust/actions/workflows/build-test.yaml)

<div align="center">
<img src="https://user-images.githubusercontent.com/40545422/230710844-2374a3ab-cbe5-49dd-8d09-bfc34b9fe6be.gif" width="1000">
</div>

## Overview
Action Clientを使用して、[navigation2](https://github.com/ros-planning/navigation2)のAction Server（`navigate_to_pose`）に目標値を送るコードをRust（[r2r](https://github.com/sequenceplanner/r2r)）実装したものです。

### Action

| **Name（Action Sever）**   | **Type**        | **Description**            | 
| ------------------- | ----------- | ---------------------- | 
| `navigate_to_pose`           | `nav2_msgs::action::NavigateToPose::Action` | ある目標値を送り、結果を取得するためのaction   | 

## 動作環境

* ROS 2 Humble
* r2r 0.7.0

# How to use

## ROS 2 Rust Install（ROS 2インストールされている前提）

```
./ros2_rust_install_script.sh 
```

## Build & Install
```
mkdir catkin_ws/src -p
cd catkin_ws
git clone git@github.com:uhobeike/nav2_send_goal_rust.git src/nav2_send_goal_rust
git clone -b humble-devel git@github.com:ROBOTIS-GIT/turtlebot3.git src/turtlebot3
git clone -b humble-devel git@github.com:ROBOTIS-GIT/turtlebot3_msgs.git src/turtlebot3_msgs
git clone -b humble-devel git@github.com:ROBOTIS-GIT/turtlebot3_simulations.git src/turtlebot3_simulations
rosdep install -r -y --from-paths --ignore-src ./ 
source /opt/ros/humble/setup.bash
source $HOME/.cargo/env
colcon  build --symlink-install
```

## Run
```
ros2 launch turtlebot3_gazebo turtlebot3_world.launch.py
ros2 launch turtlebot3_navigation2 navigation2.launch.py use_sim_time:=True
ros2 run nav2_send_goal_rust nav2_send_goal_rust
```

## Reference

### [sequenceplanner/r2r](https://github.com/sequenceplanner/r2r)
### [tier4/safe_drive_tutorial](https://github.com/tier4/safe_drive_tutorial)
