# Results

![plot](./images/result_old_spindle.png)

# Info about the measurement

- Data format for log fiels is:
  `[(seconds since UNIX epoch: f64, indicator reading in mm: f64); N]`

- Spindle spun counter clockwise. RPM is undefined, can be derived from the
  cycles of the data. Powered by DC motor through O ring gripping on the
  spindle nose.

- Z power feed used to move the milling table down (-Z) during the sweep

- One indicator was measuring Z speed, had to be reset multiple times due to
  limited travel (0.5"). This is just a reference for speed

- Total Z distance travelled from start of movement (see where ITN61000710
  starts descending to see when power feed was engaged) to fall off of data,
  259.7mm according to DRO Z axis

- Indicator was centered by finding high point at starting point

- Indicator was re-centered at the end to validate there was no significant
  change in centering (eg. mill nod). (<50um off center)

```
                     ------+     +---
                      ^    |     |
                      ||   \     / <--- Spindle
                      ||    |   | <- Starting point
                     /  \   |   |
                     |  |   |   |
                     \  /   |   | <- Test bar
    Indicator ITN61000710   |   |
    Used to calibrate Z     |   |
    speed at various        |   |       /-\
    points                  |   | c====|   | Indicator ITN61000712
             Ending point ->\___/       \-/
```

- RPM clocked to 2.5625 rpm using DFT across the entire data set

- Z power feed clocked to 59.70 um/sec

