use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use byteorder::{LittleEndian, ReadBytesExt};

fn parse_file<P: AsRef<Path>>(filename: P)
        -> io::Result<Vec<(f64, f64)>> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut results = Vec::new();
    let mut reader = &buffer[..];

    while reader.len() >= 16 {
        let time_seconds: f64 = reader.read_f64::<LittleEndian>()?;
        let indicator_reading: f64 = reader.read_f64::<LittleEndian>()?;
        results.push((time_seconds, indicator_reading));
    }

    Ok(results)
}

fn main() {
    // Find the time when the Z indicator moved the first 0.01mm. This is when
    // we consider the Z movement to start
    let offsets = parse_file("indicator_readings_ITN61000710.log")
        .unwrap();
    let mut max = std::f64::MIN;
    let mut start_time = 0.;
    for (time, mm) in offsets {
        max = max.max(mm);

        // Have we dropped 0.01mm?
        if max - mm >= 0.01 {
            start_time = time;
            break;
        }
    }

    let mut data = parse_file("indicator_readings_ITN61000712.log").unwrap();

    // Find average of data
    let mean = data.iter().map(|(_, mm)| mm)
        .sum::<f64>() / data.len() as f64;

    // Adjust all data to relative to the average, and all times to be
    // relative from the start
    data.iter_mut().for_each(|(time, mm)| {
        *time -= start_time;
        *mm   -= mean
    });

    // Discard data prior to Z travel
    //
    // Also discard data past the end of data (determined to be at 4350 sec
    // by eyeballing the graph)
    data.retain(|(time, _)| *time >= 0. && *time < 4350.);

    // Dump filtered data
    if false {
        for (seconds, mm) in &data {
            println!("{} {}", seconds, mm);
        }
        return;
    }

    // Dump DFT
    if false {
        for rpm in 1..5000 {
            let rpm = rpm as f64 / 1000.;

            // Get the frequency in hertz
            let frequency = std::f64::consts::TAU;
            let frequency = frequency * (rpm / 60.);

            let mut max = f64::MIN;
            for phase in 0..1000 {
                let phase = (phase as f64 / 1000.) * std::f64::consts::TAU;

                let mut sum = 0.;
                for (seconds, mm) in &data {
                    sum += mm * (seconds * frequency + phase).sin();
                }

                max = max.max(sum);
            }

            println!("{rpm:10.4} {max:12.6}");
        }
    }

    // Constants we've pre-computed about the data
    let rpm       = 2.5625;        // Spindle rotation
    let feed_rate = 259.7 / 4350.; // Z feed rate in mm/sec

    // Get the frequency in hertz for our sine prediction
    let frequency = std::f64::consts::TAU;
    let frequency = frequency * (rpm / 60.);

    for (ii, &(seconds, mm)) in data.iter().enumerate() {
        // Compute Z position in mm (assumed linear feed rate, it's close
        // enough)
        let z = seconds * feed_rate;

        // Scan forwards for the min and max within one wavelength
        let wavelength = 60. / rpm;

        // Find the end index for a full wavelength
        let end = seconds + wavelength;
        let mut end_idx = None;
        for (ii, &(seconds, mm)) in data.iter().enumerate().skip(ii) {
            if seconds >= end {
                end_idx = Some(ii);
                break;
            }
        }

        let Some(end_idx) = end_idx else { continue };

        // Get the slice of data in the next wavelength
        let data = &data[ii..end_idx];

        // Find the mean for the wavelength
        let mean = data.iter().map(|(_, mm)| mm)
            .sum::<f64>() / data.len() as f64;

        // Find the phase of the data
        let mut max = f64::MIN;
        let mut best_phase = 0.;
        for phase in 0..1000 {
            let phase = (phase as f64 / 1000.) * std::f64::consts::TAU;

            let mut sum = 0.;
            for (seconds, mm) in data {
                let mm = mm - mean;
                sum += mm * (seconds * frequency + phase).sin();
            }

            if sum > max {
                max = sum;
                best_phase = phase;
            }
        }

        // Find the amplitude of the data
        let mut best_amp = 0.;
        let mut min = f64::MAX;
        for amplitude in 0..1000 {
            // 0-0.1mm in 0.1um steps
            let amplitude = amplitude as f64 / 1e4;

            let mut sum_squares = 0.;
            for (seconds, mm) in data {
                let mm = mm - mean;
                let pred =
                    (seconds * frequency + best_phase).sin() *
                    (amplitude / 2.);
                sum_squares += (pred - mm) * (pred - mm);
            }

            if sum_squares < min {
                min = sum_squares;
                best_amp = amplitude;
            }
        }

        // Dump raw data
        if false {
            for (seconds, mm) in data {
                let mm = mm - mean;
                let pred =
                    (seconds * frequency + best_phase).sin() * best_amp;
            
                println!("{seconds:12.6} {pred:12.6} {mm:12.6}");
            }

            break;
        }

        println!("{z:10.4} {mm:12.6} {:12.6} {best_phase:12.6} {best_amp:12.6}", mm - mean);
    }
}

