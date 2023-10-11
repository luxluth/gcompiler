-- This is a comment

color: 0x000000; -- graph main draw color
background: 0xffffff; -- background color

grid: true; -- draw grid
grid_color: 0x000000; -- grid color
grid_alpha: 0.2; -- grid alpha
grid_thickness: 1; -- grid thickness

axes: x, y; -- axes to draw

x {
    max: 10;
    min: 0;
    name: "x";
}; -- x axis definition

y {
    max: 10;
    min: 0;
    name: "y";
}; -- y axis definition

@line {
    start: (0, 5);
    end: (7, 0);
    color: 0xff0000;
    name: "y = -x + 5";
}; -- draw a line

@graph {
    function: x^2;
    color: 0x0000ff;
    name: "y = x^2";
}; -- draw a graph of a function (x^2)

