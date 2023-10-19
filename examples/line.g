#define x
    min 0
    max 100
    name "x"
#end

#define y
    min 0
    max 100
    name "y"
#end

#root
    box 0, 0, 100, 100
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
    to 50, 100
    name "line"
    color 0xebdbb2
#end
