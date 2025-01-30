use once_cell::sync::OnceCell;
use regex::Regex;

static SEPARATOR_REGEX: OnceCell<Regex> = OnceCell::new();
static MULTI_HYPHEN_REGEX: OnceCell<Regex> = OnceCell::new();

pub fn generate_slug(name: &str) -> String {
    let name = name.to_lowercase();

    let name = name.replace('\'', "");

    // Create regex to convert separators/punctuation to hyphens
    let separator_re = SEPARATOR_REGEX.get_or_init(|| Regex::new(r"[^\w-]").unwrap());
    let with_hyphens = separator_re.replace_all(name.as_str(), "-");

    // Remove multiple consecutive hyphens
    let multi_hyphen_re = MULTI_HYPHEN_REGEX.get_or_init(|| Regex::new(r"-+").unwrap());
    let single_hyphens = multi_hyphen_re.replace_all(&with_hyphens, "-");

    // Trim hyphens from start/end
    single_hyphens.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Zeka Spur" => "zeka-spur")]
    #[test_case("Karra Karra Camp Scouting" => "karra-karra-camp-scouting")]
    #[test_case("(unnamed)" => "unnamed")]
    #[test_case("MGG #16" => "mgg-16")]
    #[test_case("Nunnett to Nowa Nowa Rd" => "nunnett-to-nowa-nowa-rd")]
    #[test_case("Alpine Way" => "alpine-way")]
    #[test_case("Bacchus Marsh -> Sunbury" => "bacchus-marsh-sunbury")]
    #[test_case("Anniversary / Gardiners / Mullum Mullum / MYT" => "anniversary-gardiners-mullum-mullum-myt")]
    #[test_case("avenel / strathbogie / alexandra / toolangi / hurstbridge" => "avenel-strathbogie-alexandra-toolangi-hurstbridge")]
    #[test_case("baw baw 23/24 - noojee detour" => "baw-baw-23-24-noojee-detour")]
    #[test_case("Baw Baw (Overnighter?)" => "baw-baw-overnighter")]
    #[test_case("December Alps Tour 🚗" => "december-alps-tour")]
    #[test_case("Almost Metro Melbourne - Mt St Leonard to Archeron Gap" => "almost-metro-melbourne-mt-st-leonard-to-archeron-gap")]
    #[test_case("Bairnsdale -> Nunnett / Nunniong -> Thredbo" => "bairnsdale-nunnett-nunniong-thredbo")]
    #[test_case("Canning river (upstream)" => "canning-river-upstream")]
    #[test_case("Hells 500 Ol\' Dirty 2014" => "hells-500-ol-dirty-2014")]
    #[test_case("Wangaratta -> Lovick\'s Hut" => "wangaratta-lovicks-hut")]
    fn test_generate_slug(input: &str) -> String {
        generate_slug(input)
    }
}
