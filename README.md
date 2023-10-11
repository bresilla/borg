



<img align="right" width="32%" src="./misc/borg_3.png">

a wannabe ros2 command replacer
===

[SUPER EARLY STATE] at the moment it calls ros2 commands externally, but the goal is to have a single binary that can be used to replace the ros2 command line tool.

```
borg <COMMAND>
eg. borg topic list
```
<hr>





## Installation

```bash
cargo install borg_ros
```

## Usage

```bash
borg <COMMAND>
```

## Commands

```
         ███        ███         ███  
       ███████    ███████     ███████
       ███████    ███████     ███████
         ███        ███         ███    

         ███        ███         ███  
       ███████    ███████     ███████
       ███████    ███████     ███████
         ███        ███         ███    
         
         ███        ███         ███  
       ███████    ███████     ███████
       ███████    ███████     ███████
         ███        ███         ███   

Usage:  borg <COMMAND>

Monotor Commands:
  action      [a]    Various action subcommands
  topic       [t]    Various topic subcommands
  service     [s]    Various service subcommands
  param       [p]    Various param subcommands
  node        [n]    Various node subcommands
  interface   [i]    Various interface subcommands
  frame       [f]    Various transforms subcommands [WIP]

Workspace Commands:
  run         [r]    Run an executable
  launch      [l]    Launch a launch file
  work        [w]    Various workspace subcommands

Utilities Commands:     
  bag         [b]    Various rosbag subcommands
  daemon      [d]    Deamon and bridge subcommands [WIP]
  middleware  [m]    Various middleware subcommands [WIP]
```