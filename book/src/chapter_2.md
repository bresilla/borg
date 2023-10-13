# Installation

## Install ROS2

### Setup Sources

You will need to add the ROS 2 apt repository to your system.
First ensure that the Ubuntu Universe repository is enabled.

```bash
sudo apt install software-properties-common
sudo add-apt-repository universe
```

Now add the ROS 2 GPG key with apt.

```bash
sudo apt update && sudo apt install curl -y
sudo curl -sSL https://raw.githubusercontent.com/ros/rosdistro/master/ros.key -o /usr/share/keyrings/ros-archive-keyring.gpg
```

Then add the repository to your sources list.

```bash
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/ros-archive-keyring.gpg] http://packages.ros.org/ros2/ubuntu $(. /etc/os-release && echo $UBUNTU_CODENAME) main" | sudo tee /etc/apt/sources.list.d/ros2.list > /dev/null
```

### Install ROS 2 packages

Update your apt repository caches after setting up the repositories.

```bash
sudo apt update
```

ROS 2 packages are built on frequently updated Ubuntu systems. It is always recommended that you ensure your system is up to date before installing new packages.

```bash
sudo apt upgrade
```

####  Desktop Install (Recommended): ROS, RViz, demos, tutorials.

```bash
sudo apt install ros-humble-desktop
```

#### ROS-Base Install (Bare Bones): Communication libraries, message packages, command line tools. No GUI tools.

```bash
sudo apt install ros-humble-ros-base
```

### Install additional RMW implementations

```bash
sudo apt install ros-humble-rmw*
```

### Development tools: Compilers and other tools to build ROS packages

```bash
sudo apt install ros-dev-tools
```

### Install foxglove and rosbridge

```bash
sudo apt install ros-humble-foxglove*
sudo apt install ros-humble-rosbridge*
```

### Install ros2 tf2 tools

```bash
sudo apt install ros-humble-tf2*
su
```

## Install BORG

### Cargo

```bash
cargo install borg_ros
```

### From Source

```bash
git clone
cd borg_ros
cargo install --path .
```