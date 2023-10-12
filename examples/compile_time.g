% compile time variables %
$var fg = 0x000000;



% compile time for loop %

$for 1..5 | i -> (
    @line
        name: ."L{i}"
        color: $fg
        from: 0, i 
        to: i, 0
    #end
)
