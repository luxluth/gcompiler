% this is a comment %
#define x
    - min 0;
    - max 100;
    - name "x";
#end;

#define y
    - min 0;
    - max 100;
    - name "y";
#end;

#root
    - box (0, 0, 100, 100);
    - color 0x000000;
    - background 0xffffff;
    - axes (x, y);
#end;

#grid
    - color 0x000000;
    - alpha 0.2;
    - thickness 1;
#end;

@line
    - from (0, 0);
    - to (100, 100);
    - name "line";
    - color 0x000000;
#end;

@graph
    - name "x^2";
    - color 0xff0000;
    - thickness 2;
    - function x^2;
#end;
