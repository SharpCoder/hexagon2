include <config.scad>

// Shape
distance_x = HEX_BORDER * 2 - 6;
distance_y = 14;

// snapfit
width = 3;

module snapfit() {
    color("blue")
    linear_extrude(SNAPFIT_DEPTH)
    rotate(180)
    mirror([1,0,0])
    translate([0,-SNAPFIT_HEIGHT,0])
    polygon([
        [0,0],
        [width,0],
        [width, .5],
        [width - 1, 2],
        [0, SNAPFIT_HEIGHT],
        [0, 0],
    ]);
}

translate([-.75,0,0])
snapfit();

color("red")
linear_extrude(SNAPFIT_DEPTH)
translate([-1.35,distance_y/2,0])
square([1.25, distance_y], center=true);


translate([distance_x+.75, 0, 0])
mirror([1,0,0])
snapfit();

translate([distance_x+SNAPFIT_HEIGHT/2, 0, 0])
color("red")
linear_extrude(SNAPFIT_DEPTH)
translate([-.625,distance_y/2,0])
square([1.25, distance_y], center=true);

color("purple")
linear_extrude(SNAPFIT_DEPTH)
translate([-SNAPFIT_HEIGHT/2, distance_y, 0])
square([distance_x+SNAPFIT_HEIGHT, 2]);
