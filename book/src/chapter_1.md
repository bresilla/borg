<p align="center">
  <img src="images/roc.png" />
</p>


## What is roc?

roc is a tool that wants to help you to develop ROS2 applications faster and easier. It is a collection of tools that are used to generate code, build and test your ROS2 applications. It is based on the [ROS2 CLI](https://index.ros.org/doc/ros2/Tutorials/Colcon-Tutorial/).

## Why roc?

roc aims to be a tool that will completely be written in RUST and at some point not relay on the ROS2 CLI anymore. This will allow us to have a tool that is faster and more reliable. It will also allow us to have a tool that can be used on other platforms like Windows.

## Features

- Generate ROS2 packages with a template system
- Build ROS2 packages (colcon at the moment) but will be replaced by a custom build system
- Adding missing feature that IMO ros2 cli should have like:
    - `roc frame` to work with tf frames, cordination systems and transformations
    - `roc bridge` to bridge topics between different ROS2 instances
- Adding TUI (Text User Interface) to make it easier to work with ROS2