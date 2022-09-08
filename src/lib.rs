pub struct Config {
    pub old_tempo: f32,
    pub new_tempo: f32,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let old_tempo = args[1].parse::<f32>();
        let new_tempo = args[2].parse::<f32>();

        if old_tempo.is_err() || new_tempo.is_err() {
            return Err("arguments must be numbers");
        }

        let old_tempo = old_tempo.unwrap();
        let new_tempo = new_tempo.unwrap();

        Ok(Config { old_tempo, new_tempo })
    }
}

pub fn run(config: Config) -> f32 {
    (config.new_tempo/config.old_tempo).log2() * 12.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twelve_semitones() {
        let test = Config { old_tempo : 140.0, new_tempo : 280.0 };
        assert_eq!(12.0, run(test));
    }
    
    #[test]
    fn no_change() {
        let test = Config { old_tempo : 140.0, new_tempo : 140.0 };
        assert_eq!(0.0, run(test));
    }
}
