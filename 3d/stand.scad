include <config.scad>;

BASE = 15;
HEIGHT = 20;

module snapfit_xtor() {
    hole_width = SNAPFIT_DEPTH;
    hole_height = SNAPFIT_HEIGHT;
    dist_y = 14;
    
    rotate([0,90,0])
    translate([-(34-12), 0, dist_y - hole_height])
    union() {
        linear_extrude(hole_height)
        square([hole_width, 200], center=true);
    }
}

difference() {
    
    //rotate([270, 180, 0])
    linear_extrude(70)
    polygon([
        [-BASE,0],
        [HEX_ARM+BASE, 0],
        [HEX_ARM, HEIGHT],
        [0, HEIGHT],
        [-BASE,0]
    ]);
    
    translate([0,-5,0])
    linear_extrude(70)
    polygon([
        [-BASE+4,0],
        [HEX_ARM+BASE-4, 0],
        [HEX_ARM-4, HEIGHT],
        [4, HEIGHT],
        [-BASE+4,0]
    ]);
    
    
    snapfit_xtor();

    translate([0,0,28])
    snapfit_xtor();
}