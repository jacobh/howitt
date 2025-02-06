use serde::{Deserialize, Serialize};
use serde_with::{formats::CommaSeparator, serde_as, StringWithSeparator};
use std::{collections::HashMap, str::FromStr};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalWeatherParams {
    pub start_date: String, // ISO8601 date
    pub end_date: String,   // ISO8601 date
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, HourlyVariable>>")]
    pub hourly: Option<Vec<HourlyVariable>>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, DailyVariable>>")]
    pub daily: Option<Vec<DailyVariable>>,
    pub latitude: f64,
    pub longitude: f64,
    pub temperature_unit: Option<TemperatureUnit>,
    pub wind_speed_unit: Option<WindSpeedUnit>,
    pub timeformat: Option<TimeFormat>,
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HourlyVariable {
    #[serde(rename = "temperature_2m")]
    Temperature2m,
    #[serde(rename = "relative_humidity_2m")]
    RelativeHumidity2m,
    #[serde(rename = "dew_point_2m")]
    DewPoint2m,
    #[serde(rename = "apparent_temperature")]
    ApparentTemperature,
    #[serde(rename = "pressure_msl")]
    PressureMsl,
    #[serde(rename = "cloud_cover")]
    CloudCover,
    #[serde(rename = "cloud_cover_low")]
    CloudCoverLow,
    #[serde(rename = "cloud_cover_mid")]
    CloudCoverMid,
    #[serde(rename = "cloud_cover_high")]
    CloudCoverHigh,
    #[serde(rename = "wind_speed_10m")]
    WindSpeed10m,
    #[serde(rename = "wind_speed_100m")]
    WindSpeed100m,
    #[serde(rename = "wind_direction_10m")]
    WindDirection10m,
    #[serde(rename = "wind_direction_100m")]
    WindDirection100m,
    #[serde(rename = "wind_gusts_10m")]
    WindGusts10m,
    #[serde(rename = "shortwave_radiation")]
    ShortwaveRadiation,
    #[serde(rename = "direct_radiation")]
    DirectRadiation,
    #[serde(rename = "direct_normal_irradiance")]
    DirectNormalIrradiance,
    #[serde(rename = "diffuse_radiation")]
    DiffuseRadiation,
    #[serde(rename = "vapour_pressure_deficit")]
    VapourPressureDeficit,
    #[serde(rename = "et0_fao_evapotranspiration")]
    Et0FaoEvapotranspiration,
    #[serde(rename = "precipitation")]
    Precipitation,
    #[serde(rename = "rain")]
    Rain,
    #[serde(rename = "weather_code")]
    WeatherCode,
    #[serde(rename = "snowfall")]
    Snowfall,
    #[serde(rename = "soil_temperature_0_to_7cm")]
    SoilTemperature0To7cm,
    #[serde(rename = "soil_temperature_7_to_28cm")]
    SoilTemperature7To28cm,
    #[serde(rename = "soil_temperature_28_to_100cm")]
    SoilTemperature28To100cm,
    #[serde(rename = "soil_temperature_100_to_255cm")]
    SoilTemperature100To255cm,
    #[serde(rename = "soil_moisture_0_to_7cm")]
    SoilMoisture0To7cm,
    #[serde(rename = "soil_moisture_7_to_28cm")]
    SoilMoisture7To28cm,
    #[serde(rename = "soil_moisture_28_to_100cm")]
    SoilMoisture28To100cm,
    #[serde(rename = "soil_moisture_100_to_255cm")]
    SoilMoisture100To255cm,
}

impl std::fmt::Display for HourlyVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_value(self).unwrap().as_str().unwrap()
        )
    }
}

impl FromStr for HourlyVariable {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyVariable {
    #[serde(rename = "temperature_2m_max")]
    Temperature2mMax,
    #[serde(rename = "temperature_2m_min")]
    Temperature2mMin,
    #[serde(rename = "apparent_temperature_max")]
    ApparentTemperatureMax,
    #[serde(rename = "apparent_temperature_min")]
    ApparentTemperatureMin,
    #[serde(rename = "precipitation_sum")]
    PrecipitationSum,
    #[serde(rename = "precipitation_hours")]
    PrecipitationHours,
    #[serde(rename = "weather_code")]
    WeatherCode,
    #[serde(rename = "sunrise")]
    Sunrise,
    #[serde(rename = "sunset")]
    Sunset,
    #[serde(rename = "wind_speed_10m_max")]
    WindSpeed10mMax,
    #[serde(rename = "wind_gusts_10m_max")]
    WindGusts10mMax,
    #[serde(rename = "wind_direction_10m_dominant")]
    WindDirection10mDominant,
    #[serde(rename = "shortwave_radiation_sum")]
    ShortwaveRadiationSum,
    #[serde(rename = "et0_fao_evapotranspiration")]
    Et0FaoEvapotranspiration,
}

impl std::fmt::Display for DailyVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_value(self).unwrap().as_str().unwrap()
        )
    }
}

impl FromStr for DailyVariable {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WindSpeedUnit {
    Kmh,
    Ms,
    Mph,
    Kn,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeFormat {
    Iso8601,
    Unixtime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalWeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i32,
    pub hourly: Option<HourlyData>,
    pub hourly_units: Option<HashMap<String, String>>,
    pub daily: Option<DailyData>,
    pub daily_units: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HourlyData {
    pub time: Vec<String>,
    pub temperature_2m: Option<Vec<f64>>,
    pub relative_humidity_2m: Option<Vec<f64>>,
    pub dew_point_2m: Option<Vec<f64>>,
    pub apparent_temperature: Option<Vec<f64>>,
    pub pressure_msl: Option<Vec<f64>>,
    pub cloud_cover: Option<Vec<f64>>,
    pub cloud_cover_low: Option<Vec<f64>>,
    pub cloud_cover_mid: Option<Vec<f64>>,
    pub cloud_cover_high: Option<Vec<f64>>,
    pub wind_speed_10m: Option<Vec<f64>>,
    pub wind_speed_100m: Option<Vec<f64>>,
    pub wind_direction_10m: Option<Vec<f64>>,
    pub wind_direction_100m: Option<Vec<f64>>,
    pub wind_gusts_10m: Option<Vec<f64>>,
    pub shortwave_radiation: Option<Vec<f64>>,
    pub direct_radiation: Option<Vec<f64>>,
    pub direct_normal_irradiance: Option<Vec<f64>>,
    pub diffuse_radiation: Option<Vec<f64>>,
    pub vapour_pressure_deficit: Option<Vec<f64>>,
    pub et0_fao_evapotranspiration: Option<Vec<f64>>,
    pub precipitation: Option<Vec<f64>>,
    pub rain: Option<Vec<f64>>,
    pub weather_code: Option<Vec<i32>>,
    pub snowfall: Option<Vec<f64>>,
    pub soil_temperature_0_to_7cm: Option<Vec<f64>>,
    pub soil_temperature_7_to_28cm: Option<Vec<f64>>,
    pub soil_temperature_28_to_100cm: Option<Vec<f64>>,
    pub soil_temperature_100_to_255cm: Option<Vec<f64>>,
    pub soil_moisture_0_to_7cm: Option<Vec<f64>>,
    pub soil_moisture_7_to_28cm: Option<Vec<f64>>,
    pub soil_moisture_28_to_100cm: Option<Vec<f64>>,
    pub soil_moisture_100_to_255cm: Option<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyData {
    pub time: Vec<String>,
    pub temperature_2m_max: Option<Vec<f64>>,
    pub temperature_2m_min: Option<Vec<f64>>,
    pub apparent_temperature_max: Option<Vec<f64>>,
    pub apparent_temperature_min: Option<Vec<f64>>,
    pub precipitation_sum: Option<Vec<f64>>,
    pub precipitation_hours: Option<Vec<f64>>,
    pub weather_code: Option<Vec<i32>>,
    pub sunrise: Option<Vec<String>>,
    pub sunset: Option<Vec<String>>,
    pub wind_speed_10m_max: Option<Vec<f64>>,
    pub wind_gusts_10m_max: Option<Vec<f64>>,
    pub wind_direction_10m_dominant: Option<Vec<f64>>,
    pub shortwave_radiation_sum: Option<Vec<f64>>,
    pub et0_fao_evapotranspiration: Option<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: bool,
    pub reason: String,
}
