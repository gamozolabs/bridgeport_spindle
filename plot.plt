set terminal qt size 1440,900

set xtics 10
set ytics 1
set grid

set xrange [0:]
set yrange [0:]

set title "Spindle runout on the Bridgeport (original spindle)"

set xlabel "Z offset from spindle nose (mm)"
set ylabel "Runout (um)"

# curve fitting
f(x) = sqrt(a**2 + (b*x + c)**2)
fit f(x) "log.txt" using 1:($5 * 1e3) via a,b,c
fit_title = sprintf("y=%2.4f x=%2.4f*z+%2.4f", a, b, c)

plot "log.txt" u 1:($5 * 1e3) w l title "raw data", \
    f(x) title fit_title

pause -1

