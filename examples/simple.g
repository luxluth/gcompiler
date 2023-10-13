% this is a comment %
#define x
    min 0
    max 200
    name "x"
#end

#define y
    min 0
    max 200
    name "y"
#end

#root
    box 0, 0, 200, 200
    color 0xebdbb2
    background 0x282828
    axis x, y
#end

#grid
    color 0x3c3836
    step 10
    alpha 1
#end

@line
    from 0, 0
    to 50, 50
    name "line"
    color 0xebdbb2
#end

@graph
    name "x^2"
    color 0xff0000
    thickness 2
    func "x^2"
#end

@point
    at 50, 50
    name "A"
    color 0x0000ff
#end
