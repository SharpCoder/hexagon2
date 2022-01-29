include <config.scad>;
$fn = 6;

module snapfit_xtor() {
    hole_width = SNAPFIT_DEPTH;
    hole_height = SNAPFIT_HEIGHT;
    dist_y = 12;
    
    translate([14, 0, dist_y-hole_height])
    linear_extrude(hole_height)
    square([hole_width, 200], center=true);
}

module all_sides() {
    children();
    rotate(60)
    children();
    rotate(120)
    children();
}

// Make an interior strengthening wall
difference() {
    union() {
        linear_extrude(HEX_HEIGHT - SHIELD_THICKNESS)
        difference() {
            $fn = 6;
            circle(d=HEX_WIDTH);
            circle(d=HEX_WIDTH - 3);
            
        }

        difference() {
            linear_extrude(HEX_HEIGHT)
            circle(d=HEX_WIDTH + HEX_BORDER);

            linear_extrude(HEX_HEIGHT + 1)
            circle(d=HEX_WIDTH);

        }
    }

    all_sides()
    union() {
        snapfit_xtor();
        
        mirror([1,0,0])
        snapfit_xtor();
    }
}