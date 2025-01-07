// check .osu documentation to check what fields should be given Option<T>

#[derive(Default, Debug)]
pub struct OsuBeatmap {
    version: String,
    general: General,
    editor: Editor,
    metadata: Metadata,
    difficulty: Difficulty,
    events: Events,
    timing_points: Vec<TimingPoint>,
    hit_objects: Vec<Vec<String>>,
}

#[derive(Default, Debug)]
struct General {
    audio: String,
    audio_lead_in: i32,
    preview_time: i32,
    countdown: i32,
    sample_set: String,
    stack_leniency: f32,
    mode: i32,
    letterbox_in_breaks: bool,
    widescreen_storyboard: bool,
}

#[derive(Default, Debug)]
struct Editor {
    bookmarks: Vec<i32>,
    distance_spacing: f32,
    beat_divisor: i32,
    grid_size: i32,
    timeline_zoom: f32,
}

#[derive(Default, Debug)]
struct Metadata {
    title: String,
    title_unicode: String,
    artist: String,
    artist_unicode: String,
    creator: String,
    version: String,
    source: String,
    tags: Vec<String>,
    beatmap_id: i32,
    beatmap_set_id: i32,
}

#[derive(Default, Debug)]
struct Difficulty {
    hp_drain_rate: f32,
    circle_size: f32,
    overall_difficulty: f32,
    approach_rate: f32,
    slider_multiplier: f32,
    slider_tick_rate: f32,
}

#[derive(Default, Debug)]
struct Events {
    bg_src: String,
}

#[derive(Debug)]
struct TimingPoint {
    time: i32,
    beat_length: f32,
    meter: i32,
    sample_set: i32,
    sample_index: i32,
    volume: i32,
    uninherited: bool,
    effects: String, // properly parse this in the future, related to kiai
}

// Fix this in the future to properly handle sliders
struct HitObject {
    x: i32,
    y: i32,
    time: i32,
    r#type: i32,
    hit_sound: i32,
    object_params: Vec<String>,
    hit_sample: Vec<i32>,
}

enum CurrentSection {
    Default,
    General,
    Editor,
    Metadata,
    Difficulty,
    Events,
    TimingPoints,
    Colours,
    HitObjects,
}

struct KeyValue {
    key: String,
    value: String,
}

impl KeyValue {
    fn new(line: &str) -> Self {
        let split: Vec<&str> = line.splitn(2, ':').collect();

        KeyValue {
            key: split[0].trim().to_string(),
            value: split[1].trim().to_string(),
        }
    }
}

fn int_to_bool(num: i32) -> bool {
    match num {
        0 => false,
        1 => true,
        _ => false,
    }
}

pub fn parse_beatmap(path: &str) -> Result<OsuBeatmap, String> {
    // Read file contents
    let values = std::fs::read_to_string(path).map_err(|_| String::from("Failed to read file"))?;

    let lines: Vec<&str> = values.lines().filter(|line| !line.is_empty()).collect();

    let mut current_section: CurrentSection = CurrentSection::Default;

    let mut beatmap = OsuBeatmap::default();

    for line in lines {
        // Set the section of the .osu that is being read
        match line {
            "[General]" => {
                current_section = CurrentSection::General;
                continue;
            }

            "[Editor]" => {
                current_section = CurrentSection::Editor;
                continue;
            }

            "[Metadata]" => {
                current_section = CurrentSection::Metadata;
                continue;
            }

            "[Difficulty]" => {
                current_section = CurrentSection::Difficulty;
                continue;
            }

            "[Events]" => {
                current_section = CurrentSection::Events;
                continue;
            }

            "[TimingPoints]" => {
                current_section = CurrentSection::TimingPoints;
                continue;
            }

            "[Colours]" => {
                current_section = CurrentSection::Colours;
                continue;
            }

            "[HitObjects]" => {
                current_section = CurrentSection::HitObjects;
                continue;
            }

            _ => {}
        }

        match current_section {
            CurrentSection::Default => {
                let version = line
                    .split(' ')
                    .nth(3)
                    .ok_or("Missing version field")?
                    .strip_prefix('v')
                    .ok_or("Version formatting is invalid")?;

                if version != "14" {
                    return Err("Only handling v14 beatmaps for now".to_string());
                }

                beatmap.version = version.to_string();
            }

            CurrentSection::General => {
                let key_value = KeyValue::new(line);

                match key_value.key.as_str() {
                    "AudioFilename" => beatmap.general.audio = key_value.value,
                    "AudioLeadIn" => {
                        beatmap.general.audio_lead_in = key_value.value.parse::<i32>().unwrap_or(0)
                    }
                    "PreviewTime" => {
                        beatmap.general.preview_time = key_value.value.parse::<i32>().unwrap_or(0)
                    }
                    "Countdown" => {
                        beatmap.general.countdown = key_value.value.parse::<i32>().unwrap_or(0)
                    }
                    "SampleSet" => beatmap.general.sample_set = key_value.value,
                    "StackLeniency" => {
                        beatmap.general.stack_leniency =
                            key_value.value.parse::<f32>().unwrap_or(0.0)
                    }
                    "Mode" => beatmap.general.mode = key_value.value.parse::<i32>().unwrap_or(0),
                    "LetterboxInBreaks" => {
                        beatmap.general.letterbox_in_breaks =
                            int_to_bool(key_value.value.parse::<i32>().unwrap_or(0))
                    }
                    "WidescreenStoryboard" => {
                        beatmap.general.widescreen_storyboard =
                            int_to_bool(key_value.value.parse::<i32>().unwrap_or(0))
                    }
                    _ => {
                        eprintln!("Key didn't match: {:?}", key_value.key)
                    }
                }
            }

            CurrentSection::Editor => {
                let key_value = KeyValue::new(line);

                match key_value.key.as_str() {
                    "Bookmarks" => {
                        beatmap.editor.bookmarks = key_value
                            .value
                            .split(',')
                            .map(|v| v.parse::<i32>().unwrap_or(0))
                            .collect();
                    }

                    "DistanceSpacing" => {
                        beatmap.editor.distance_spacing =
                            key_value.value.parse::<f32>().unwrap_or(0.0)
                    }

                    "BeatDivisor" => {
                        beatmap.editor.beat_divisor = key_value.value.parse::<i32>().unwrap_or(0)
                    }
                    "GridSize" => {
                        beatmap.editor.grid_size = key_value.value.parse::<i32>().unwrap_or(0)
                    }
                    "TimelineZoom" => {
                        beatmap.editor.timeline_zoom = key_value.value.parse::<f32>().unwrap_or(0.0)
                    }
                    _ => {
                        eprintln!("Key didn't match: {:?}", key_value.key)
                    }
                }
            }

            CurrentSection::Metadata => {
                let key_value = KeyValue::new(line);

                match key_value.key.as_str() {
                    "Title" => beatmap.metadata.title = key_value.value,
                    "TitleUnicode" => beatmap.metadata.title_unicode = key_value.value,
                    "Artist" => beatmap.metadata.artist = key_value.value,
                    "ArtistUnicode" => beatmap.metadata.artist_unicode = key_value.value,
                    "Creator" => beatmap.metadata.creator = key_value.value,
                    "Version" => beatmap.metadata.version = key_value.value,
                    "Source" => beatmap.metadata.source = key_value.value,
                    "Tags" => {
                        beatmap.metadata.tags =
                            key_value.value.split(' ').map(|s| s.to_string()).collect()
                    }
                    "BeatmapID" => {
                        beatmap.metadata.beatmap_id = key_value.value.parse::<i32>().unwrap_or(0)
                    }
                    "BeatmapSetID" => {
                        beatmap.metadata.beatmap_set_id =
                            key_value.value.parse::<i32>().unwrap_or(0)
                    }
                    _ => {
                        eprintln!("Key didn't match: {:?}", key_value.key)
                    }
                }
            }

            CurrentSection::Difficulty => {
                let key_value = KeyValue::new(line);

                match key_value.key.as_str() {
                    "HPDrainRate" => {
                        beatmap.difficulty.hp_drain_rate =
                            key_value.value.parse::<f32>().unwrap_or(0.0)
                    }
                    "CircleSize" => {
                        beatmap.difficulty.circle_size =
                            key_value.value.parse::<f32>().unwrap_or(0.0)
                    }
                    "OverallDifficulty" => {
                        beatmap.difficulty.overall_difficulty =
                            key_value.value.parse::<f32>().unwrap_or(0.0)
                    }
                    "ApproachRate" => {
                        beatmap.difficulty.approach_rate =
                            key_value.value.parse::<f32>().unwrap_or(0.0)
                    }
                    "SliderMultiplier" => {
                        beatmap.difficulty.slider_multiplier =
                            key_value.value.parse::<f32>().unwrap_or(0.0)
                    }
                    "SliderTickRate" => {
                        beatmap.difficulty.slider_tick_rate =
                            key_value.value.parse::<f32>().unwrap_or(0.0)
                    }
                    _ => {
                        eprintln!("Key didn't match: {:?}", key_value.key)
                    }
                }
            }

            CurrentSection::Events => {
                let char_vec: Vec<char> = line.chars().collect();
                match char_vec[0] {
                    '0' => {
                        if beatmap.events.bg_src.is_empty() {
                            let split: Vec<&str> = line.split(",").collect();
                            beatmap.events.bg_src = split[2].replace('"', "");
                        }
                    }
                    _ => continue,
                }
            }

            CurrentSection::TimingPoints => {
                let split: Vec<&str> = line.split(",").collect();

                if split.len() != 8 {
                    eprintln!("Timing point invalid or something: {:?}", line);
                    continue;
                }

                let timing_point = TimingPoint {
                    time: split[0].parse::<i32>().unwrap_or(0),
                    beat_length: split[1].parse::<f32>().unwrap_or(0.0),
                    meter: split[2].parse::<i32>().unwrap_or(0),
                    sample_set: split[3].parse::<i32>().unwrap_or(0),
                    sample_index: split[4].parse::<i32>().unwrap_or(0),
                    volume: split[5].parse::<i32>().unwrap_or(0),
                    uninherited: int_to_bool(split[6].parse::<i32>().unwrap_or(0)),
                    effects: split[7].to_string(),
                };

                beatmap.timing_points.push(timing_point);
            }

            CurrentSection::Colours => {}

            CurrentSection::HitObjects => {
                let split: Vec<String> = line.split(",").map(|s| s.to_string()).collect();
                beatmap.hit_objects.push(split);
            }
        }
    }

    Ok(beatmap)
}
