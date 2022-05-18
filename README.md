# rieasm

A program that converts `.rie` files into RLEs to paste into Golly.

# Usage

1. make sure you have [Golly](https://sourceforge.net/projects/golly/files/) and [Rust](https://www.rust-lang.org/tools/install) installed.

2. Clone this repo.

3. Copy `golly/Flow6.rule` into your rules folder. Open up `golly/Turing Machine.mc`. It should look something like this:

   | ![image](https://user-images.githubusercontent.com/49224759/169102690-671830f1-47ef-4f2c-a58e-61fc94749c04.png) |
   |:--:|
   | The Flow6 Turing Machine. |

4. Go back to your terminal and type `cargo run program --clip`. This will first compile the rieasm assembler, which will then run, compiling the `program.rie` file found in this repository into your clipboard.
   | ![image](https://user-images.githubusercontent.com/49224759/169112146-7200754b-b4c9-4317-bfb1-db79eb675fb6.png) |
   |:--:|
   | Terminal should look something like this after running the command. |


5. Go back to Golly, and follow these instructions to paste the RLE:

  | ![image](https://user-images.githubusercontent.com/49224759/169105264-ee759a54-9f00-42d0-9187-622c06228fb4.png) |
  |:--:|
  | Hover and scroll to zoom. No need to hold any buttons. |
  | ![image](https://user-images.githubusercontent.com/49224759/169109031-2c85e079-807b-443d-beeb-13ed224b257a.png) |
  | Align your crosshair with the tile marked with green. It should say `XY=0 0` on the top bar. |
  | Once the cursor is aligned, hit Ctrl+V and left click. |

6. Set the step size. Press the `+` and `-` keys on your keyboard until it's the right speed.
   | ![image](https://user-images.githubusercontent.com/49224759/169106753-1dfaa5f7-6b77-4293-a4a0-f0155a62a35b.png) |
   |:--:|
   | I recommend `8^0` if you want to track the exact paths of the signals. |
   | `8^1` if you want to see individual register operations. |
   | `8^2` if you want to see what the whole program does. |

7. Run the simulation by clicking the Green Play button on the top left.
   | ![image](https://user-images.githubusercontent.com/49224759/169111090-f57aa923-8391-41a2-b970-225990f00878.png) |
   |:--:|
   | ![image](https://user-images.githubusercontent.com/49224759/169110580-d5408b94-1b32-4017-97a1-443d71b454b9.png) |
   | The red button will pause the simulation, while the blue button will reset it. |
