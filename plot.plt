set terminal qt size 1440,900

set xtics 10
set ytics 1
set grid

set xrange [0:]
set yrange [0:]

set title "Spindle runout on the Bridgeport"

set xlabel "Z offset from spindle nose (mm)"
set ylabel "Runout (um)"

set nokey

plot "log.txt" u 1:($5 * 1e3) w l

pause -1

