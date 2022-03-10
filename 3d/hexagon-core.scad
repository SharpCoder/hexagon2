include <pcb-mount.scad>;

$fn = 6;

module hexagon_core() {
    module snapfit_xtor() {
        hole_width = SNAPFIT_DEPTH;
        hole_height = SNAPFIT_HEIGHT;
        dist_y = 22;
        
        translate([14, 0, dist_y - hole_height])
        union() {
            linear_extrude(hole_height)
            square([hole_width, 200], center=true);
        }
    }

    module wire_channel() {
        translate([14, 0, 11.6])
        translate([-14, 0, -11])
        linear_extrude(8)
        square([10, 1000], center=true);
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
                circle(d=HEX_WIDTH - 4);
                
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
            
            wire_channel();
        }
    }


    // Floor
    color("red")
    linear_extrude(1.6)
    circle(d=HEX_WIDTH + HEX_BORDER, $fn=6);

    translate([0, 0, 0])
    pcb_mount();    
}

//hexagon_core();

