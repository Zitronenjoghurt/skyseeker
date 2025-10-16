use anyhow::Context;
use serde::{Deserialize, Serialize};
use skyseeker_core::celestial_body::star::Star;
use skyseeker_core::celestial_body::CelestialBody;
use skyseeker_core::math::{
    angle_format_to_radians, arc_seconds_to_radians, time_format_to_radians,
};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct BSC5Entry {
    /// Harvard Revised Number = Bright Star Number
    #[serde(alias = "HR")]
    pub hr: String,
    /// Name, generally bayer and/or Flamsteed name
    #[serde(alias = "Name")]
    pub name: Option<String>,
    #[serde(alias = "Common")]
    pub common: Option<String>,
    #[serde(alias = "Bayer")]
    pub bayer: Option<String>,
    #[serde(alias = "BayerF")]
    pub bayer_full: Option<String>,
    /// Constellation, if it is in one
    #[serde(alias = "Constellation")]
    pub constellation: Option<String>,
    #[serde(default, alias = "Notes")]
    pub notes: Vec<BSC5EntryNote>,
    /// Equinox J2000 right ascension, hours
    #[serde(alias = "RAh")]
    pub right_ascension_hours: String,
    /// Equinox J2000 right ascension, minutes
    #[serde(alias = "RAm")]
    pub right_ascension_minutes: String,
    /// Equinox J2000 right ascension, seconds
    #[serde(alias = "RAs")]
    pub right_ascension_seconds: String,
    /// Equinox J2000 declination, sign + or -
    #[serde(alias = "DE-")]
    pub declination_sign: String,
    /// Equinox J2000 declination, degrees
    #[serde(alias = "DEd")]
    pub declination_degrees: String,
    /// Equinox J2000 declination, arcminutes
    #[serde(alias = "DEm")]
    pub declination_minutes: String,
    /// Equinox J2000 declination, arcseconds
    #[serde(alias = "DEs")]
    pub declination_seconds: String,
    /// Visual magnitude in mag
    #[serde(alias = "Vmag")]
    pub visual_magnitude: String,
    /// Proper motion in right ascension from equinox J2000 in arcseconds per year
    #[serde(alias = "pmRA")]
    pub proper_motion_right_ascension: String,
    /// Proper motion in declination from equinox J2000 in arcseconds per year
    #[serde(alias = "pmDE")]
    pub proper_motion_declination: String,
    /// Heliocentric radial velocity in km/s
    #[serde(alias = "RadVel")]
    pub heliocentric_radial_velocity: Option<String>,
    /// Parallax in arcseconds
    #[serde(alias = "Parallax")]
    pub parallax: Option<String>,
    /// B-V color in the UBV system
    #[serde(alias = "B-V")]
    pub b_v_color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BSC5EntryNote {
    #[serde(alias = "Category")]
    pub category: String,
    #[serde(alias = "Remark")]
    pub remark: String,
}

pub fn parse(data: String) -> anyhow::Result<Vec<CelestialBody>> {
    let entries =
        serde_json::from_str::<Vec<BSC5Entry>>(&data).context("Failed to deserialize BSC5 data")?;

    let stars = entries
        .into_iter()
        .filter_map(|entry| {
            (|| -> anyhow::Result<CelestialBody> {
                let hr = entry.hr.parse::<u16>().context("failed to parse HR")?;

                let right_ascension = parse_right_ascension(
                    entry.right_ascension_hours,
                    entry.right_ascension_minutes,
                    entry.right_ascension_seconds,
                )
                .context("failed to parse right ascension")?;

                let declination = parse_declination(
                    entry.declination_sign,
                    entry.declination_degrees,
                    entry.declination_minutes,
                    entry.declination_seconds,
                )
                .context("failed to parse declination")?;

                let proper_motion_right_ascension_arcs = entry
                    .proper_motion_right_ascension
                    .parse::<f64>()
                    .context("failed to parse proper motion")?;
                let proper_motion_right_ascension =
                    arc_seconds_to_radians(proper_motion_right_ascension_arcs);

                let proper_motion_declination_arcs = entry
                    .proper_motion_declination
                    .parse::<f64>()
                    .context("failed to parse proper motion")?;
                let proper_motion_declination =
                    arc_seconds_to_radians(proper_motion_declination_arcs);

                let parallax = entry
                    .parallax
                    .unwrap_or("0.001".to_string())
                    .parse::<f64>()
                    .context("failed to parse parallax")?;

                let radial_velocity = entry
                    .heliocentric_radial_velocity
                    .context("missing radial velocity")?
                    .parse::<f64>()
                    .context("failed to parse radial velocity")?;

                let visual_magnitude = entry
                    .visual_magnitude
                    .parse::<f64>()
                    .context("failed to parse visual magnitude")?;

                let b_v_color = if let Some(b_v_color) = entry.b_v_color {
                    Some(
                        b_v_color
                            .parse::<f64>()
                            .context("failed to parse B-V color")?,
                    )
                } else {
                    None
                };

                let star = Star {
                    id: format!("HR {}", hr),
                    hr: Some(hr),
                    name: entry.name,
                    common_name: entry.common,
                    bayer: entry.bayer,
                    bayer_full: entry.bayer_full,
                    constellation: entry.constellation,
                    notes: entry
                        .notes
                        .into_iter()
                        .map(|note| (note.category, note.remark))
                        .collect(),
                    right_ascension,
                    declination,
                    proper_motion_right_ascension,
                    proper_motion_declination,
                    parallax,
                    radial_velocity,
                    visual_magnitude,
                    b_v_color,
                };

                Ok(CelestialBody::Star(Arc::new(star)))
            })()
            .inspect_err(|e| println!("Skipping entry 'HR = {}' in BSC5: {}", entry.hr, e))
            .ok()
        })
        .collect();

    Ok(stars)
}

fn parse_right_ascension(hours: String, minutes: String, seconds: String) -> anyhow::Result<f64> {
    let hours = hours.parse::<u32>()?;
    let minutes = minutes.parse::<u32>()?;
    let seconds = seconds.parse::<f64>()?;
    Ok(time_format_to_radians(' ', hours, minutes, seconds)?)
}

fn parse_declination(
    sign: String,
    degrees: String,
    minutes: String,
    seconds: String,
) -> anyhow::Result<f64> {
    let sign = sign.parse::<char>()?;
    let degrees = degrees.parse::<u32>()?;
    let minutes = minutes.parse::<u32>()?;
    let seconds = seconds.parse::<f64>()?;

    Ok(angle_format_to_radians(sign, degrees, minutes, seconds)?)
}
