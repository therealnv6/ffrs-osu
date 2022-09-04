use std::any::Any;
use std::boxed::Box;

use crate::macros::{Parsed, read_value};
use convert_case::{Case, Casing};

crate::macros::parsed!(GeneralMetadata {
    audio_file_name    : String = AudioFilename,
    audio_lead_in      : i64    = Empty,
    preview_time       : i64    = Empty,
    countdown          : i8     = Empty,
    sample_set         : String = Empty,
    stack_leniency     : f32    = Empty,
    mode               : i8     = Empty,
    letterbox_in_breaks: i8     = Empty
});

crate::macros::parsed!(EditorMetadata {
    distance_spacing: f32 = Empty,
    beat_divisor    : i8  = Empty,
    grid_size       : i8  = Empty
});

crate::macros::parsed!(DifficultyMetadata {
    hp_drain_rate     : i8 = Empty,
    circle_size       : i8 = Empty,
    overall_difficulty: i8 = Empty,
    slider_multiplier : i8 = Empty,
    slider_tick_rate  : i8 = Empty
});

struct TimingPointsMetadata {
    points: Vec<Box<dyn TimingPoint>>,
}

impl Parsed for TimingPointsMetadata {
    fn is_section_id(id: String) -> bool {
        return id == "[TimingPoints]";
    }

    fn parse_from(section: Vec<String>) -> Self
        where
            Self: Sized {

        let mut points: Vec<Box<dyn TimingPoint>> = Vec::new();

        for line in section {
            let spliced = line.split(",").collect::<Vec<&str>>();

            let x = spliced[0].parse::<i32>().unwrap();
            let y = spliced[1].parse::<i32>().unwrap();
            let time = spliced[2].parse::<i32>().unwrap();
            let hit_sound = spliced[3].parse::<i8>().unwrap();
            let extra_data = spliced[4..]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            points.push(
                Box::new(CircleTimingPoint::parse_from(x, y, time, hit_sound, extra_data))
            )
        }

        Self {
            points
        }
    }
}

trait TimingPoint {
    fn parse_from(x: i32, y: i32, time: i32, hit_sound: i8, extra_data: Vec<String>) -> Self
        where Self: Sized;
    fn get_dimension(&self) -> (i32, i32);
    fn get_time(&self) -> i32;
    fn get_type(&self) -> i8;
    fn get_hit_sound(&self) -> i8;
    fn get_object_parameters(&self) -> Vec<ObjectParam>;
    fn get_hit_sample(&self) -> Vec<HitSample>; 
}

#[derive(Clone, Debug)]
struct CircleTimingPoint {
    x: i32,
    y: i32,
    time: i32,
    hit_sound: i8,
}

impl TimingPoint for CircleTimingPoint {
    fn parse_from(x: i32, y: i32, time: i32, hit_sound: i8, _: Vec<String>) -> Self {
        Self {
            x,
            y,
            time,
            hit_sound
        }
    }

    fn get_dimension(&self) -> (i32, i32) {
        return (self.x, self.y);
    }

    fn get_hit_sample(&self) -> Vec<HitSample> {
        return vec![];
    }

    fn get_hit_sound(&self) -> i8 {
        return self.hit_sound;
    }

    fn get_object_parameters(&self) -> Vec<ObjectParam> {
        return vec![];
    }

    fn get_time(&self) -> i32 {
        return self.time;
    }

    fn get_type(&self) -> i8 {
        return 0;
    }
}

struct ObjectParam;
struct HitSample;