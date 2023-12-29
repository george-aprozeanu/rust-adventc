use crate::types::MyResult;
use std::cmp;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}
impl Category {
    fn of(value: &str) -> Category {
        match value {
            "seed" => Category::Seed,
            "soil" => Category::Soil,
            "fertilizer" => Category::Fertilizer,
            "water" => Category::Water,
            "light" => Category::Light,
            "temperature" => Category::Temperature,
            "humidity" => Category::Humidity,
            "location" => Category::Location,
            _ => panic!("category?"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Range {
    from: i64,
    size: i64,
}

impl Range {
    fn to(&self) -> i64 {
        return self.from + self.size - 1;
    }
    fn move_by(&self, value: i64) -> Range {
        Range {
            from: self.from + value,
            size: self.size,
        }
    }
    fn project(&self, dest: i64) -> Range {
        Range {
            from: dest,
            size: self.size,
        }
    }
    fn intersect(&self, range: &Range) -> Option<Range> {
        Range::start_end(
            cmp::max(self.from, range.from),
            cmp::min(self.to(), range.to()),
        )
    }
    fn prefix(&self, range: &Range) -> Option<Range> {
        (self.from < range.from)
            .then(|| ())
            .and_then(|_| Range::start_end(
                self.from,
                cmp::min(self.to(), range.to()),
            ))
    }
    fn substract(&self, range: &Range) -> Vec<Range> {
        if let Some(inter) = self.intersect(&range) {
            vec![]
        } else {
            vec![self.clone()]
        }
    }
    fn start_end(start: i64, end: i64) -> Option<Range> {
        (start <= end).then(|| Range {
            from: start,
            size: end + 1 - start,
        })
    }
    fn from_size(from: i64, size: i64) -> Option<Range> {
        (size > 0).then(|| Range {
            from,
            size: size.try_into().unwrap(),
        })
    }
    fn tr_val(&self, value: i64, dest: i64) -> Option<i64> {
        (self.from <= value && value <= self.to()).then(|| value + dest - self.from)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Translation {
    range: Range,
    dest: i64,
}

impl Translation {
    fn from_str(vec: &Vec<&str>) -> MyResult<Translation> {
        let from = vec[1].parse()?;
        let size = vec[2].parse()?;
        let dest = vec[0].parse()?;
        Ok(Translation {
            range: Range::from_size(from, size).ok_or("range?")?,
            dest,
        })
    }
    fn tr_val(&self, val: i64) -> Option<i64> {
        self.range.tr_val(val, self.dest)
    }
    fn join(&self, other: &Translation) -> Vec<Translation> {
        let project = self.range.project(self.dest);
        let inter = project.intersect(&other.range);
        let inter_join = inter
            .map(|inter| inter.move_by(self.range.from - project.from))
            .map(|inter| Translation {
                range: inter,
                dest: other.dest,
            });
        let outer = project.substract(&other.range);

        let projection_start = self.dest;
        let inter_start = cmp::max(projection_start, other.range.from);
        let inter_join_start = inter_start - projection_start + self.range.from;
        let prefix_size = inter_join_start - self.range.from;

        [
            inter_join,
            (prefix_size > 0).then(|| Translation {
                range: Range {
                    from: self.range.from,
                    size: prefix_size,
                },
                dest: self.dest,
            }),
        ]
        .into_iter()
        .filter_map(|a| a)
        .collect()
    }
}

#[test]
fn test_translation_join() {
    let that = Translation {
        range: Range { from: 1, size: 8 },
        dest: 8,
    };
    let other = Translation {
        range: Range { from: 12, size: 40 },
        dest: 100,
    };
    let result = that.join(&other);
    assert_eq!(
        result,
        [Translation {
            range: Range { from: 5, size: 4 },
            dest: 100
        }]
    );
}

struct TranslationMap<'a> {
    seed: &'a TranslationCategory,
    soil: &'a TranslationCategory,
    fertilizer: &'a TranslationCategory,
    water: &'a TranslationCategory,
    light: &'a TranslationCategory,
    temperature: &'a TranslationCategory,
    humidity: &'a TranslationCategory,
    translations: Translations,
}

impl TranslationMap<'_> {
    fn filter(
        input_maps: &Vec<TranslationCategory>,
        category: Category,
    ) -> MyResult<&TranslationCategory> {
        Ok(input_maps
            .iter()
            .find(|input_map| input_map.category.eq(&category))
            .ok_or("seed?")?)
    }
    fn of(categories: &Vec<TranslationCategory>) -> MyResult<TranslationMap> {
        let seed = TranslationMap::filter(categories, Category::Seed)?;
        let soil = TranslationMap::filter(categories, Category::Soil)?;
        let fertilizer = TranslationMap::filter(categories, Category::Fertilizer)?;
        let water = TranslationMap::filter(categories, Category::Water)?;
        let light = TranslationMap::filter(categories, Category::Light)?;
        let temperature = TranslationMap::filter(categories, Category::Temperature)?;
        let humidity = TranslationMap::filter(categories, Category::Humidity)?;

        let translations = seed
            .translations
            .join(&soil.translations)
            .join(&fertilizer.translations)
            .join(&water.translations)
            .join(&light.translations)
            .join(&temperature.translations)
            .join(&humidity.translations);

        Ok(TranslationMap {
            seed,
            soil,
            fertilizer,
            water,
            light,
            temperature,
            humidity,
            translations,
        })
    }
    fn tr(&self, seed: i64) -> i64 {
        let soil = self.seed.tr(seed);
        let fertilizer = self.soil.tr(soil);
        let water = self.fertilizer.tr(fertilizer);
        let light = self.water.tr(water);
        let temperature = self.light.tr(light);
        let humidity = self.temperature.tr(temperature);
        let location = self.humidity.tr(humidity);
        location
    }

    fn tr_range(&self, seed_range: &Range) -> Option<Range> {
        // let soil = self.seed.tr_range(&seed_range);
        // let fertilizer = soil.and_then(|soil| self.soil.tr_range(&soil));
        // let water = fertilizer.and_then(|fertilizer| self.fertilizer.tr_range(&fertilizer));
        // let light = water.and_then(|water| self.water.tr_range(&water));
        // let temperature = light.and_then(|light| self.light.tr_range(&light));
        // let humidity = temperature.and_then(|temperature| self.temperature.tr_range(&temperature));
        // let location = humidity.and_then(|humidity| self.humidity.tr_range(&humidity));
        // println!("== soil {:?}\n fertilizer {:?}\n water {:?}\n light {:?}\n temperature {:?}\n humidity {:?}\n location {:?}",
        // soil, fertilizer, water, light, temperature, humidity, location);
        // location
        None
    }
}

#[derive(Debug)]
struct Translations {
    translations: Vec<Translation>,
}

impl Translations {
    fn join_one(&self, other: &Translation) -> Vec<Translation> {
        self.translations
            .iter()
            .flat_map(|that| that.join(&other))
            .collect()
    }
    fn join(&self, other: &Translations) -> Translations {
        let translations: Vec<Translation> = other
            .translations
            .iter()
            .flat_map(|other| self.join_one(other))
            .collect();
        Translations { translations }
    }
    fn tr_val(&self, val: i64) -> Option<i64> {
        self.translations.iter().find_map(|pair| pair.tr_val(val))
    }
}

#[test]
fn test_translations_join_one() {
    let that = Translations {
        translations: vec![
            Translation {
                range: Range { from: 1, size: 4 },
                dest: 100,
            },
            Translation {
                range: Range { from: 10, size: 4 },
                dest: 200,
            },
        ],
    };
    let other = Translation {
        range: Range {
            from: 102,
            size: 100,
        },
        dest: 1000,
    };
    let result = that.join_one(&other);
    assert_eq!(
        result,
        [
            Translation {
                range: Range { from: 3, size: 2 },
                dest: 1000
            },
            Translation {
                range: Range { from: 10, size: 2 },
                dest: 1000
            }
        ]
    );
}

#[derive(Debug)]
struct TranslationCategory {
    category: Category,
    translations: Translations,
}

impl TranslationCategory {
    fn read(
        lines: &mut impl Iterator<Item = io::Result<String>>,
    ) -> MyResult<Option<TranslationCategory>> {
        if let Some(title_line) = lines.next() {
            let title_line = title_line?;
            let from_sep = title_line.find("-to-").ok_or("'-to-' ?")?;
            let from = &title_line[0..from_sep];
            let mut translations: Vec<Translation> = vec![];
            loop {
                if let Some(line) = lines.next() {
                    let line = line?;
                    if line.len() == 0 {
                        break;
                    }
                    let parts: Vec<&str> = line.splitn(3, ' ').collect();
                    translations.push(Translation::from_str(&parts)?);
                } else {
                    break;
                }
            }
            Ok(Some(TranslationCategory {
                category: Category::of(from),
                translations: Translations { translations },
            }))
        } else {
            Ok(None)
        }
    }
    fn tr(&self, val: i64) -> i64 {
        self.translations.tr_val(val).get_or_insert(val).to_owned()
    }
}

fn seeds(line: &str) -> MyResult<Vec<i64>> {
    let space = line.find(' ').ok_or("space?")?;
    let seed_str = &line[space + 1..];
    let seeds: Vec<i64> = seed_str
        .split(' ')
        .map(|a| a.parse())
        .collect::<Result<_, _>>()?;
    return Ok(seeds);
}

pub fn p1(file: &str) -> MyResult<()> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let line = lines.next().ok_or("first?")??;
    let seeds = seeds(line.as_ref())?;
    lines.next().ok_or("next?")??;
    let mut categories: Vec<TranslationCategory> = vec![];
    loop {
        if let Some(map) = TranslationCategory::read(&mut lines)? {
            categories.push(map);
        } else {
            break;
        }
    }
    let translation_map = TranslationMap::of(&categories)?;
    let min = seeds
        .iter()
        .map(|val| translation_map.tr(val.to_owned()))
        .min()
        .ok_or("min?")?;
    println!("day5p1 {}", min);

    Ok(())
}

fn seed_pairs(line: &str) -> MyResult<Vec<Range>> {
    let space = line.find(' ').ok_or("space?")?;
    let seed_str = &line[space + 1..];
    let (_, ranges) = seed_str.split(' ').map(|a| a.parse::<i64>()).try_fold(
        (None, vec![]),
        |(prev, vec), next| -> MyResult<(Option<i64>, Vec<Range>)> {
            match prev {
                Some(prev) => Ok((
                    None,
                    [vec, vec![Range::from_size(prev, next?).ok_or("size?")?]].concat(),
                )),
                None => Ok((Some(next?), vec)),
            }
        },
    )?;
    return Ok(ranges);
}

pub fn p2(file: &str) -> MyResult<()> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let line = lines.next().ok_or("first?")??;
    let seed_pairs = seed_pairs(line.as_ref())?;
    lines.next().ok_or("next?")??;
    let mut translation_maps: Vec<TranslationCategory> = vec![];
    loop {
        if let Some(map) = TranslationCategory::read(&mut lines)? {
            translation_maps.push(map);
        } else {
            break;
        }
    }
    let translation_map = TranslationMap::of(&translation_maps)?;
    println!("{:?} translations", translation_map.translations);
    Ok(())
}
