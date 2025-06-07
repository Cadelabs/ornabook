use std::fmt::Write;
use std::path::Path;

use saphyr::{LoadableYamlNode, YamlOwned};

type AnguishPath = Vec<[Vec<(String, String)>; 4]>;

/// Anguish choices for each path.
///
/// ```text
/// Vec<[  Vec<(String, String)>; 4]>
/// ^   ^  ^   ^
/// |   |  |   The (modifier, value) pair
/// |   |  Array of modifiers
/// |   Array of 4 choices
/// Array of path levels
///
/// i.e.: despair[level][choice][modifier].0 (modifier)
///       despair[level][choice][modifier].1 (value)
/// ```
#[derive(Default, Debug)]
pub struct AnguishChoices {
    pub despair: AnguishPath,
    pub melancholy: AnguishPath,
    pub agony: AnguishPath,
    pub torment: AnguishPath,
}

impl AnguishChoices {
    /// Load the choices from the given file.
    ///
    /// # Panic
    /// This function panics if anything goes wrong :)
    /// Be thankful I spare you the error handling, future me.
    pub fn from_yaml<P: AsRef<Path>>(path: P) -> Self {
        let yaml =
            YamlOwned::load_from_str(&std::fs::read_to_string(path.as_ref()).unwrap()).unwrap();
        let yaml = &yaml[0]; // First document
        Self {
            despair: parse_path(&yaml["Despair"]),
            melancholy: parse_path(&yaml["Melancholy"]),
            agony: parse_path(&yaml["Agony"]),
            torment: parse_path(&yaml["Torment"]),
        }
    }
}

/// Load the choices of a single path.
///
/// # Panic
/// This function panics if anything goes wrong.
fn parse_path(yaml: &YamlOwned) -> AnguishPath {
    let mut ret = vec![];
    for level in yaml.as_sequence().unwrap() {
        let key = format!("Anguish {}", ret.len() + 1);
        let choices = level.as_mapping_get(&key).unwrap();
        ret.push([
            parse_choice(choices[0].as_mapping_get("Choice 1").unwrap()),
            parse_choice(choices[1].as_mapping_get("Choice 2").unwrap()),
            parse_choice(choices[2].as_mapping_get("Choice 3").unwrap()),
            parse_choice(choices[3].as_mapping_get("Choice 4").unwrap()),
        ]);
    }

    ret
}

/// Load the modifiers of a single choice.
///
/// # Panic
/// This function panics if anything goes wrong.
fn parse_choice(yaml: &YamlOwned) -> Vec<(String, String)> {
    let mut ret = vec![];
    for modifier in yaml.as_sequence().unwrap() {
        let modifier = modifier.as_mapping().unwrap();
        let (modifier, value) = modifier.iter().next().unwrap();
        ret.push((
            modifier.as_str().unwrap().to_string(),
            value.as_str().unwrap().to_string(),
        ));
    }
    ret
}

/// Build the HTML table of choices for the given path.
pub fn make_table(path: &AnguishPath, path_name: &str) -> String {
    let mut html = format!(
        r#"<table class="anguish-choices" id="ang-{path_name}-choices">
  <thead> <tr> <th>Lv</th> <th>Choice 1</th> <th>Choice 2</th> <th>Choice 3</th> <th>Choice 4</th> </tr> </thead>
  <tbody style="text-align: center;">"#
    );
    for (idx, level) in path.iter().enumerate() {
        write!(html, "<tr><td>{}</td>", idx + 1).unwrap();
        for choice in level {
            write!(html, "<td>").unwrap();
            for (modifier, value) in choice {
                write!(
                    html,
                    r#"<span class="{}">{modifier}: {value}</span><br/>"#,
                    get_modifier_class(modifier),
                )
                .unwrap();
            }
            write!(html, "</td>").unwrap();
        }
        write!(html, "</tr>").unwrap();
    }
    write!(html, "</tbody></table>").unwrap();

    html
}

/// Get the class (color) of the span for the given modifier.
fn get_modifier_class(modifier: &str) -> &'static str {
    match modifier {
        "Enemy stats" => "ang-stats",
        "EXP Bonus"
        | "Orn Bonus"
        | "Gold Bonus"
        | "Tower Shard Bonus"
        | "Item Drop Quality Bonus"
        | "Proof of Despair Chance"
        | "Proof of Melancholy Chance"
        | "Proof of Agony Chance"
        | "Proof of Torment Chance"
        | "Ang. Gear Chance" => "ang-bonus",
        "Damage from Undead"
        | "Damage from Magic Foes"
        | "Damage from Nothren"
        | "Damage from Dragons"
        | "Damage from Beasts"
        | "Damage from Ancients"
        | "Damage from Mimic"
        | "Status Protection"
        | "Crit Chance"
        | "Crit Damage"
        | "Fire Damage"
        | "Water Damage"
        | "Lightning Damage"
        | "Earthen Damage"
        | "Holy Damage"
        | "Dark Damage"
        | "Dragon Damage"
        | "Arcane Damage"
        | "Multi-target Damage"
        | "Accuracy"
        | "Permanent Status Effect Fade Chance"
        | "Defend Power"
        | "1HP Instead of Kill Chance"
        | "Berserk Encounters"
        | "Healing" => "ang-malus",
        _ => panic!("Unknown modifier {modifier}"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        use crate::AnguishChoices;
        dbg!(AnguishChoices::from_yaml(
            "../orna_data/anguish-choices.yaml"
        ));
    }
}
