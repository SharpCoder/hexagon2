include <config.scad>;
$fn = 6;

module magnet_holder() {
    W = 60.5;
    H = 3.5;
    P = 2;
    
    translate([0, -H + (HEX_WIDTH - HEX_ARM + HEX_BORDER)/2, 0])
    difference() {
        linear_extrude(HEX_HEIGHT - SHIELD_THICKNESS)
        square([W+P, H+P], center=true);
        
        translate([0, 0, HEX_HEIGHT-12-3])
        linear_extrude(HEX_HEIGHT-4)
        square([W, H], center=true);
    }
}

module circuit_arm() {
    difference() {
        rotate(90)
        translate([HEX_WIDTH - 20 - 26, -5,0])
        linear_extrude(6)
        square([ 20, 10]);
        
        linear_extrude(4)
        translate([0, 40, 0])
        circle(r=4.2, $fn = 6);
        
        $fn = 100;
        linear_extrude(100)
        translate([0, 40, 0])
        circle(r=3);
        
    }
}

//circuit_arm();

// Make an interior strengthening wall
linear_extrude(5)
difference() {
    $fn = 6;
    circle(d=HEX_WIDTH);
    circle(d=HEX_WIDTH - 14);
}

difference() {
    linear_extrude(HEX_HEIGHT)
    circle(d=HEX_WIDTH + HEX_BORDER);

    linear_extrude(HEX_HEIGHT + 1)
    circle(d=HEX_WIDTH);

    /*
    rotate(180)
    bore();
    bore();
    */
}

// Neodynum magnet for scale
rotate(-60)
magnet_holder();

mirror([0,1,0])
magnet_holder();

//rotate(60)
//magnet_holder();

mirror([0,1,0])
rotate(120)
magnet_holder();

//mirror([1,0,0])
//rotate(60)
//magnet_holder();

//rotate(240)
//magnet_holder();