set terminal qt size 1440,900
set y2tics
set grid xtics ytics y2tics

set xdata time
set timefmt '"%Y-%m-%d %H:%M:%S"'
set format x "%Y-%m-%d\n%H:%M:%S"

set nokey

plot "indicator_readings_ITN61000712.log" binary format="%double%double" u 1:($2 * 1e3) w l
pause -1

