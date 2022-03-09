# hexagon2
Hexagon wall v2

![Final product](https://mudon.s3.us-west-2.amazonaws.com/hexwall.jpg)

## Installation

To properly build the hexagon2 project, you'll need the following:

```bash
# Install build tools
sudo apt-get install gcc-arm-none-eabi jq

# Configure rust
rustup default nightly
rustup target add thumbv7em-none-eabi
```


## Building

In order for your project to build correctly, you'll need to run the following command:

```
./build.sh
```

The build script will generate a `.hex` file and place it in a folder called `out`. This hex file is compatible with the teensy 4.0 and can be flashed with the teensy-loader utility.

**CAUTION**: Do not build this in release mode. It optimizes a lot of stuff away, and can cause problems.


## Bill of Materials

### Brain

 - 1x [DC Barrel Jack](https://www.sparkfun.com/products/10811)
 - 1x DS18B20
 - 1x Resistor
 - 1x 100uf Capacitor
 - 1x [ESP8266](https://www.amazon.com/dp/B010N1ROQS)
 - 1x [Teensy4.0](https://www.sparkfun.com/products/16997)
 - 3x [WS2812b SMD](https://www.sparkfun.com/products/16346)

### Unit

 - 3x [WS2812b SMD](https://www.sparkfun.com/products/16346)
 - 2x Right-Angle Headers

### Hardware (per unit)

 - 1x M4x12 Bolt
 - 1x M4 Nut
 - 2x M4 Washers
 

### Circuits
There are two circuit boards which need to be fabricated. The gerber files are located in the `gerber/` folder. You will need:

 - 1x brain_v1.0.zip
 - nx unit_v1.1.zip

You need 1 brain per project and as many units as you like. Each hexagon will require 1 unit.

*NOTE* There is SMD soldering required for this project.


### 3D Printed

There are a few parts which need to be 3D printed. You can find everything in the `3d/` folder. The important things are pre-generated in .STL format. You can also find all the openscad source code for each object.

 - hexcore.stl
 - hexshield.stl
 - snapfit.stl

---
### hexcore.stl
This is the actual hexagon which you will mount the circuitry to.

**Infill** 15 or 20%

**Pattern** Rectilinear is fine

**Material** PLA Recommended

**Color** White

---

### hexshield.stl
This is the faceplate which attaches to each hexagon node using a friction-fit mechanism.

**Infill** 100%

**Pattern** Rectilinear **Required**

**Material** PETG Recommended

**Color** Transparent

---

### snapfit.stl
This is the little snapfit connector that you can use to attach hexagons together.

**Infill** 15 or 20%

**Pattern** Rectilinear is fine

**Material** PLA recommended

**Color** Any

---


## License
[MIT](https://choosealicense.com/licenses/mit/)