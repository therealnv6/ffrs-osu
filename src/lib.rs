mod data;
mod macros;

#[cfg(test)]
mod tests {
    use crate::data::GeneralMetadata;
    use crate::macros::Parsed;
    use std::num::ParseIntError;

    #[test]
    fn it_works() {
        let metadata = GeneralMetadata::parse_from(
            vec![
                "AudioFilename: Niko - Made of Fire.mp3",
                "AudioLeadIn: 1000",
                "PreviewTime: 40703",
                "Countdown: 1",
                "SampleSet: Normal",
                "StackLeniency: 0.7",
                "Mode: 0",
                "LetterboxInBreaks: 1",
            ]
            .iter()
            .cloned()
            .map(String::from)
            .collect(),
        );

        assert!(metadata.is_ok());
        assert_eq!(
            metadata.unwrap(),
            GeneralMetadata {
                audio_file_name: Some("Niko - Made of Fire.mp3".to_owned()),
                audio_lead_in: Some(1000),
                preview_time: Some(40703),
                countdown: Some(1),
                sample_set: Some("Normal".to_owned()),
                stack_leniency: Some(0.7),
                mode: Some(0),
                letterbox_in_breaks: Some(1)
            }
        );
    }
}
