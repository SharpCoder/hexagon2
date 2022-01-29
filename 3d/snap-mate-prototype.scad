

// Shape
width = 25;
height = 16;
length = 15.4 - 5;

// Hole
hole_width = 12.5;
hole_height = 7.5;


difference() {
    linear_extrude(height)
    square([width,length], center=true);
    
    translate([0,0,4])
    linear_extrude(hole_height)
    square([hole_width, length+1], center=true);
}