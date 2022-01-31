include <config.scad>

// Shape
distance_x = 16.1;//HEX_BORDER * 2 - 6;
distance_y = 17.5;

// snapfit
width = 4;
SNAPFIT_DEPTH = 6.45;
tol = .5;
module snapfit() {
    color("blue")
    linear_extrude(SNAPFIT_DEPTH)
    rotate(180)
    mirror([1,0,0])
    translate([-width,-SNAPFIT_HEIGHT,0])
    polygon([
        [0,0],
        [width, 0],
        [width-1, 2],
        [0, SNAPFIT_HEIGHT],
        [0, 0],
    ]);
}


module assembly() {
    translate([-distance_x / 2, 0, 0])
    difference() {
        union() {
            translate([-tol-.5, 0, 0])
            mirror([1,0,0])
            snapfit();

            translate([distance_x + tol - .25, 0, 0])
            mirror([0,0,0])
            snapfit();
            
            color("red")
            linear_extrude(SNAPFIT_DEPTH)
            translate([width - 1.5, 0, 0])
            square([2, distance_y]);
            
            color("red")
            linear_extrude(SNAPFIT_DEPTH)
            translate([distance_x - width - 1.5 , 0, 0])
            square([2, distance_y]);
            
            color("purple")
            linear_extrude(SNAPFIT_DEPTH)
            translate([(distance_x)/2, distance_y + .75, 0])
            square([2+distance_x*1.25, 2.5], center=true);
        }
        
        linear_extrude(SNAPFIT_DEPTH+1)
        translate([distance_x/2-.5,2 + distance_y/2,0])
        scale([1.0,2.0,3.0]) 
        circle(d=distance_x/2, $fn = 100);
        
        linear_extrude(10)
        square([200, .75], center=false);

    }
}

assembly();

translate([-42+distance_x,0,0])
assembly();

translate([-distance_x+2.5,distance_y+.75,0])
linear_extrude(SNAPFIT_DEPTH)
square([45.5,2.5], center=true);
