use std::{
    cmp::{max, min},
    collections::HashSet,
};

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

pub struct Sensor {
    pub x: isize,
    pub y: isize,
    pub reach: usize,
}

impl Sensor {
    pub fn x_range(&self, y: isize) -> Option<(isize, isize)> {
        let diff_y = (self.y - y).unsigned_abs() as usize;

        if diff_y >= self.reach {
            return None;
        }

        let reach_at_y = self.reach - diff_y;
        let min_x = self.x - reach_at_y as isize;
        let max_x = self.x + reach_at_y as isize;

        Some((min_x, max_x))
    }
}

#[derive(Default)]
pub struct ExclusionZone {
    pub x: isize,
    pub y: isize,
    pub width: usize,
    pub height: usize,
    pub beacons: HashSet<(isize, isize)>,
    pub sensors: Vec<Sensor>,
}

impl ExclusionZone {
    pub fn find_beacon(
        &self,
        from_x: isize,
        from_y: isize,
        to_x: isize,
        to_y: isize,
    ) -> Option<(isize, isize)> {
        (from_y..=to_y).into_par_iter().find_map_any(|at_y| {
            let mut ranges: Vec<(isize, isize)> = self
                .sensors
                .iter()
                .filter_map(|sensor| {
                    let Some(mut reachable_range) = sensor.x_range(at_y) else {
                    return None
                };

                    reachable_range.0 = reachable_range.0.clamp(from_x, to_x);
                    reachable_range.1 = reachable_range.1.clamp(from_x, to_x);
                    Some(reachable_range)
                })
                .collect();

            ranges = merge_ranges(ranges);

            if ranges.len() > 1 {
                let (x1, x2) = ranges[0];
                let (x3, x4) = ranges[1];

                let mut values = vec![x1, x2, x3, x4];
                values.sort();

                Some(((values[1] + 1), at_y))
            } else {
                None
            }
        })
    }

    pub fn math_find_beacon(&self) -> Option<(isize, isize)> {
        let mut positive_lines = vec![];
        let mut negative_lines = vec![];

        for sensor in &self.sensors {
            positive_lines.push(sensor.x - sensor.y - sensor.reach as isize);
            positive_lines.push(sensor.x - sensor.y + sensor.reach as isize);
            negative_lines.push(sensor.x + sensor.y - sensor.reach as isize);
            negative_lines.push(sensor.x + sensor.y + sensor.reach as isize);
        }

        let mut range = 0..self.sensors.len() * 2;

        let Some(positive_line) = range.find_map(|i| {
            let positive_line = ((i + 1)..self.sensors.len() * 2).find_map(|j| {
                let a = positive_lines[i];
                let b = positive_lines[j];

                if (a - b).abs() == 2 {
                    return Some(a.min(b) + 1)
                }
                None
            });

            let Some(positive) = positive_line else {
                return None;
            };

            Some(positive)

        }) else {
            return None;
        };

        let Some(negative_line) = range.find_map(|i| {
            let negative_line = ((i + 1)..self.sensors.len() * 2).find_map(|j| {
                let a = negative_lines[i];
                let b = negative_lines[j];

                if (a - b).abs() == 2 {
                    return Some(a.min(b) + 1)
                }
                None
            });

            let Some(negative) = negative_line else {
                return None;
            };

            Some(negative)

        }) else {
            return None;
        };

        Some((
            (positive_line + negative_line) / 2,
            (negative_line - positive_line) / 2,
        ))
    }

    pub fn exclusion_count(&self, at_y: isize) -> usize {
        let mut ranges = vec![];

        for sensor in &self.sensors {
            let Some(reachable_range) = sensor.x_range(at_y) else {
                continue;
            };

            ranges.push(reachable_range);
        }

        ranges = merge_ranges(ranges);

        let mut beacons = 0;

        for (from, to) in &ranges {
            for (x, y) in &self.beacons {
                if *y == at_y && x >= from && x <= to {
                    beacons += 1;
                }
            }
        }

        let mut size = ranges.into_iter().fold(0, |acc, (a, b)| {
            if a < 0 && b > 0 {
                b - a + acc + 1
            } else {
                b - a + acc
            }
        });

        size -= beacons;

        size as usize
    }
}

impl From<&str> for ExclusionZone {
    fn from(value: &str) -> Self {
        let mut output = Self::default();
        let mut max_x = 0;
        let mut max_y = 0;

        for line in value.lines() {
            let (mut sensor_str, mut beacon_str) = line.split_once(": ").unwrap();
            sensor_str = sensor_str.strip_prefix("Sensor at ").unwrap();
            let (sensor_x_str, sensor_y_str) = sensor_str.split_once(", ").unwrap();

            let sensor_x: isize = sensor_x_str.strip_prefix("x=").unwrap().parse().unwrap();
            let sensor_y: isize = sensor_y_str.strip_prefix("y=").unwrap().parse().unwrap();

            beacon_str = beacon_str.strip_prefix("closest beacon is at ").unwrap();

            let (beacon_x_str, beacon_y_str) = beacon_str.split_once(", ").unwrap();

            let beacon_x: isize = beacon_x_str.strip_prefix("x=").unwrap().parse().unwrap();
            let beacon_y: isize = beacon_y_str.strip_prefix("y=").unwrap().parse().unwrap();

            let manhattan_distance =
                ((sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs()) as usize;

            max_x = max_x.max(sensor_x).max(beacon_x);
            max_y = max_y.max(sensor_y).max(beacon_y);

            output.x = output.x.min(sensor_x).min(beacon_x);
            output.y = output.y.min(sensor_y).min(beacon_y);

            let sensor = Sensor {
                x: sensor_x,
                y: sensor_y,
                reach: manhattan_distance,
            };

            output.sensors.push(sensor);
            output.beacons.insert((beacon_x, beacon_y));
        }

        let width = (max_x - output.x) as usize + 1;
        let height = (max_y - output.y) as usize + 1;

        output.width = width;
        output.height = height;

        output
    }
}

fn merge_ranges(ranges: Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    let mut sorted_ranges = ranges;
    sorted_ranges.sort_by_key(|range| range.0);

    let mut merged_ranges = Vec::new();
    let mut current_range = sorted_ranges[0];
    sorted_ranges.into_iter().for_each(|range| {
        if range.0 > current_range.1 {
            merged_ranges.push(current_range);
            current_range = range;
        } else {
            current_range = (min(current_range.0, range.0), max(current_range.1, range.1));
        }
    });
    merged_ranges.push(current_range);

    merged_ranges
}

pub fn solve_1(input: &str) -> String {
    let zone = ExclusionZone::from(input);

    if cfg!(test) {
        zone.exclusion_count(10).to_string()
    } else {
        zone.exclusion_count(2000000).to_string()
    }
}
pub fn solve_2(input: &str) -> String {
    let zone = ExclusionZone::from(input);

    if cfg!(test) {
        let beacon = zone.math_find_beacon().expect("find beacon");

        let tuning_frequency = beacon.0 * 4000000 + beacon.1;

        tuning_frequency.to_string()
    } else {
        let beacon = zone
            .find_beacon(0, 0, 4000000, 4000000)
            .expect("find beacon");
        //let beacon = zone.math_find_beacon().expect("find beacon");

        let tuning_frequency = beacon.0 * 4000000 + beacon.1;

        tuning_frequency.to_string()
    }
}

#[test]
fn merge_range_test() {
    let ranges = vec![(1, 5), (2, 6), (8, 10), (15, 20), (17, 19)];
    let merged_ranges = merge_ranges(ranges);

    assert_eq!(merged_ranges, [(1, 6), (8, 10), (15, 20)])
}
