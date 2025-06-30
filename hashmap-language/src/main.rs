use clap::Parser;
use core::f64;
use std::collections::HashMap;

struct LanguageStats {
    creation_year: i32,
    users_millions: f64,
}

#[derive(Debug, Parser)]
struct Cli {
    #[clap(short, long)]
    lang: Option<String>,
}

fn init_languages() -> HashMap<String, LanguageStats> {
    let mut languages = HashMap::new();
    languages.insert(
        "JavaScript".to_string(),
        LanguageStats {
            creation_year: 1995,
            users_millions: 12.0,
        },
    );
    languages.insert(
        "HTML/CSS".to_string(),
        LanguageStats {
            creation_year: 1990,
            users_millions: 8.0,
        },
    );
    languages.insert(
        "Python".to_string(),
        LanguageStats {
            creation_year: 1991,
            users_millions: 10.0,
        },
    );
    languages.insert(
        "SQL".to_string(),
        LanguageStats {
            creation_year: 1974,
            users_millions: 5.0,
        },
    );
    languages.insert(
        "TypeScript".to_string(),
        LanguageStats {
            creation_year: 2012,
            users_millions: 2.0,
        },
    );
    languages.insert(
        "Bash/Shell".to_string(),
        LanguageStats {
            creation_year: 1989,
            users_millions: 3.0,
        },
    );
    languages.insert(
        "Java".to_string(),
        LanguageStats {
            creation_year: 1995,
            users_millions: 10.0,
        },
    );
    languages.insert(
        "C#".to_string(),
        LanguageStats {
            creation_year: 2000,
            users_millions: 5.0,
        },
    );
    languages.insert(
        "C++".to_string(),
        LanguageStats {
            creation_year: 1985,
            users_millions: 7.0,
        },
    );
    languages.insert(
        "C".to_string(),
        LanguageStats {
            creation_year: 1972,
            users_millions: 4.0,
        },
    );
    languages.insert(
        "PHP".to_string(),
        LanguageStats {
            creation_year: 1995,
            users_millions: 8.0,
        },
    );
    languages.insert(
        "PowerShell".to_string(),
        LanguageStats {
            creation_year: 2006,
            users_millions: 2.0,
        },
    );
    languages.insert(
        "Go".to_string(),
        LanguageStats {
            creation_year: 2007,
            users_millions: 1.0,
        },
    );
    languages.insert(
        "Rust".to_string(),
        LanguageStats {
            creation_year: 2010,
            users_millions: 1.5,
        },
    );

    languages
}

fn calculate_weights(years_active: &mut HashMap<String, LanguageStats>) -> HashMap<String, i32> {
    // Subtract the creation year from 2024 to get the number of years active.
    for stat in years_active.values_mut() {
        stat.creation_year = 2024 - stat.creation_year;
    }

    let min_year = years_active
        .values()
        .map(|stat| stat.creation_year)
        .min()
        .unwrap_or(0);
    let max_year = years_active
        .values()
        .map(|stat| stat.creation_year)
        .max()
        .unwrap_or(0);

    let min_users = years_active
        .values()
        .map(|stat| stat.users_millions)
        .fold(f64::INFINITY, |a, b| a.min(b));

    let max_users = years_active
        .values()
        .map(|stat| stat.users_millions)
        .fold(f64::INFINITY, |a, b| a.max(b));

    let mut weights = HashMap::new();
    let age_weight = 0.7;
    let users_weight = 0.3;

    for (language, stats) in years_active.iter() {
        let year = stats.creation_year;
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let users = stats.users_millions;
        let normalized_users = (users - min_users) / (max_users - min_users);
        let combine_score = (normalized_year * age_weight) + (normalized_users * users_weight);
        let weight = (combine_score * 99.0) as i32 + 1; // weight between 1 and 100
        weights.insert(language.to_string(), weight);
    }

    weights
}

fn main() {
    let cli = Cli::parse();

    let mut languages = init_languages();
    if let Some(lang) = cli.lang {
        if let Some((lang, stat)) = lang.split_once("-") {
            if let Some((year_str, users_str)) = stat.split_once(",") {
                if let (Ok(year), Ok(users)) = (year_str.parse::<i32>(), users_str.parse::<f64>()) {
                    languages.insert(
                        lang.to_string(),
                        LanguageStats {
                            creation_year: year,
                            users_millions: users,
                        },
                    );
                }
            }
        }
    }
    let weights = calculate_weights(&mut languages);

    let mut weights_sorted: Vec<(String, i32)> = weights.into_iter().collect();
    weights_sorted.sort_by(|a, b| a.1.cmp(&b.1));

    println!("Language weighing from 1-100 by age and popularity:");
    for (language, weight) in &weights_sorted {
        let stats = languages.get(language).unwrap();

        println!(
            "{language}: {weight} (Age: {} years, Users: {:.1}M)",
            stats.creation_year, stats.users_millions
        );
    }
}
