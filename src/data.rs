use crate::data::timing_point::TimingPoint;
use crate::macros::field_parser;
use crate::parsing::{FieldParser, ParseError, Parsed};
use std::boxed::Box;

use crate::macros::read_value;
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

field_parser!(i64);
field_parser!(i32);
field_parser!(i16);
field_parser!(i8);
field_parser!(f32);
field_parser!(f64);

/// Manually implemented, parsed struct.
/// Will retrieve all keys annotated in the [TimingPoints] section, following the
/// (Hit Objects)[https://osu.ppy.sh/wiki/en/Client/File_formats/Osu_%28file_format%29#hit-objects] section within the osu documentation.
///
/// This, regardless of being manually implemented, still implements
/// the [Parsed] trait, and therefore [Parsed::parse_from()] can be used.
/// Our implementation for [TimingPointsMetadata] will do the following:
/// - Split all of the shared hit-object related data:
///   - x: i32
///   - y: i32
///   - time: i32
///   - hit_sound: i32
/// - All other remaining data will be manually parsed through the [TimingPoint] trait.
struct TimingPointsMetadata {
    points: Vec<Box<dyn crate::data::timing_point::TimingPoint>>,
}

impl Parsed for TimingPointsMetadata {
    fn is_section_id(id: String) -> bool {
        return id == "[TimingPoints]";
    }

    fn parse_from(section: Vec<String>) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let mut points: Vec<Box<dyn crate::data::timing_point::TimingPoint>> = Vec::new();

        for line in section {
            let spliced = line.split(",").collect::<Vec<&str>>();

            let x: i32 = spliced[0].parse_field()?;
            let y: i32 = spliced[1].parse_field()?;
            let time: i32 = spliced[2].parse_field()?;
            let ty: i8 = spliced[3].parse_field()?;
            let hit_sound: i8 = spliced[4].parse_field()?;
            let extra_data = spliced[5..]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            let parse = |i| {
                match i {
                    0 => crate::data::timing_point::CircleTimingPoint::parse_from(
                        x, y, time, hit_sound, extra_data,
                    ),
                    _ => todo!(), // todo: implement others
                }
            };

            points.push(Box::new(parse(ty)?))
        }

        Ok(Self { points })
    }
}

mod timing_point {
    use crate::parsing::{FieldParser, ParseError};

    pub trait TimingPoint {
        fn parse_from(
            x: i32,
            y: i32,
            time: i32,
            hit_sound: i8,
            extra_data: Vec<String>,
        ) -> Result<Self, ParseError>
        where
            Self: Sized;
        fn get_dimension(&self) -> (i32, i32);
        fn get_time(&self) -> i32;
        fn get_type(&self) -> i8;
        fn get_hit_sound(&self) -> i8;
        fn get_object_parameters(&self) -> Vec<ObjectParam>;
        fn get_hit_sample(&self) -> Vec<HitSample>;
    }

    #[derive(Clone, Debug)]
    pub struct CircleTimingPoint {
        x: i32,
        y: i32,
        time: i32,
        hit_sound: i8,
    }

    #[derive(Clone, Debug)]
    pub struct SliderTimingPoint {
        x: i32,
        y: i32,
        time: i32,
        hit_sound: i8,
        curve_type: i8,
        curve_points: Vec<CurvePoint>,
        slides: i8,
        length: f32,
        edge_sounds: Vec<i8>,
        edge_sets: Vec<String>,
    }

    #[derive(Clone, Debug)]
    pub struct CurvePoint {
        x: i32,
        y: i32,
    }

    impl CurvePoint {
        fn parse(str: &String) -> Result<Self, ParseError> {
            let split = str
                .split(":")
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            let x: i32 = split[0].parse_field()?;
            let y: i32 = split[1].parse_field()?;

            Ok(Self { x, y })
        }
    }

    impl TimingPoint for SliderTimingPoint {
        fn parse_from(
            x: i32,
            y: i32,
            time: i32,
            hit_sound: i8,
            data: Vec<String>,
        ) -> Result<Self, ParseError> {
            let first_pipe = data[0]
                .split("|")
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            let curve_type = first_pipe[0].parse_field()?;
            let curve_points = first_pipe[1..]
                .into_iter()
                .map(CurvePoint::parse)
                .flatten()
                .collect::<Vec<CurvePoint>>();

            let slides = data[1].parse_field()?;
            let length = data[2].parse_field()?;

            Ok(Self {
                x,
                y,
                time,
                hit_sound,
            })
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

    impl TimingPoint for CircleTimingPoint {
        fn parse_from(
            x: i32,
            y: i32,
            time: i32,
            hit_sound: i8,
            _: Vec<String>,
        ) -> Result<Self, ParseError> {
            Ok(Self {
                x,
                y,
                time,
                hit_sound,
            })
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

    pub struct ObjectParam;
    pub struct HitSample;
}
