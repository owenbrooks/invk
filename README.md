# invk
Experimenting with inverse kinematics algorithms

![Demo GIF](./demo.gif)

Works with 2D planar revolute-joint robots

Provides two binaries:

- `direct.rs` to demonstrate the display of a robotic arm with joint angles varying over time
- `inverse.rs` which uses the inverse kinematics algorithm to set the end of the arm to the coordinates of the mouse. See screenshot.

Uses either [FABRIK](http://www.andreasaristidou.com/FABRIK.html) or a gradient-descent algorithm to compute the inverse kinematics numerically.
The graphics display is done using [nannou](https://github.com/nannou-org/nannou).

# How to run
- Install [rust](https://www.rust-lang.org/learn/get-started)
- Clone the repository:
`git clone https://github.com/owenbrooks/invk.git`
- Change directory: `cd invk`
- Run using
`cargo run --release --bin inverse` or `cargo run --release --bin direct`

