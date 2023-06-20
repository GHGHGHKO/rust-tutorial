pub enum Weather {
    Rain,
    Snowing,
    Foggy,
    Earthquake,
    Typhoon
}

impl Weather {
    pub fn weather_forecasting_stone(&self) -> &str {
        match self {
            Self::Rain => "Stone is wet",
            Self::Snowing => "White on top",
            Self::Foggy => "Can't see stone",
            Self::Earthquake => "Stone jumping up&down",
            Self::Typhoon => "Stone gone"
        }
    }
}
