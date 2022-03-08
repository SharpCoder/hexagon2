include <config.scad>;

BASE = 15;
HEIGHT = 20;

module snapfit_xtor() {
    hole_width = SNAPFIT_DEPTH;
    hole_height = SNAPFIT_HEIGHT;
    dist_y = 3;
    
    rotate([270,0,0])
    translate([-(25-12), 0, dist_y - hole_height])
    union() {
        linear_extrude(hole_height)
        square([hole_width, 200], center=true);
    }
}

difference() {
    linear_extrude(20)
    square([65,38], center=true);



    snapfit_xtor();

    translate([28,0,0])
    snapfit_xtor();
}