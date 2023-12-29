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

#[derive(Clone, Copy, Debug)]
struct Range {
    from: u64,
    size: u64,
}

impl Range {
    fn to(&self) -> u64 {
        return self.from + self.size - 1;
    }
    fn intersect(&self, range: &Range) -> Option<Range> {
        Range::start_end(
            cmp::max(self.from, range.from),
            cmp::min(self.to(), range.to()),
        )
    }
    fn start_end(start: u64, end: u64) -> Option<Range> {
        (start <= end).then(|| Range {
            from: start,
            size: end + 1 - start,
        })
    }
    fn start_size(from: u64, size: u64) -> Option<Range> {
        (size > 0).then(|| Range { from, size })
    }
    fn tr(&self, value: u64, dest: u64) -> Option<u64> {
        (self.from <= value && value <= self.to()).then(|| value + dest - self.from)
    }

    fn tr_range(&self, range: &Range, dest: u64) -> Range {
        Range {
            from: range.from + dest - self.from,
            size: range.size,
        }
    }
}

#[derive(Clone, Debug)]
struct TranslationRange {
    source: Range,
    dest: u64,
}

impl TranslationRange {
    fn from(vec: &Vec<&str>) -> MyResult<TranslationRange> {
        let from = vec[1].parse()?;
        let size = vec[2].parse()?;
        let dest = vec[0].parse()?;
        Ok(TranslationRange {
            source: Range::start_size(from, size).ok_or("range?")?,
            dest,
        })
    }
    fn tr(&self, val: u64) -> Option<u64> {
        self.source.tr(val, self.dest)
    }
    fn tr_range(&self, range: &Range) -> Option<Range> {
        self.source
            .intersect(range)
            .map(|intr| self.source.tr_range(&intr, self.dest))
    }

    fn join(&self, tr_range: &TranslationRange) -> Option<TranslationRange> {
        self.tr_range(&tr_range.source)
            .map(|inter| TranslationRange::from_range(inter, self.dest))
    }

    fn from_range(source: Range, dest: u64) -> TranslationRange {
        TranslationRange { source, dest }
    }
}

struct InputSet<'a> {
    seed: &'a InputMap,
    soil: &'a InputMap,
    fertilizer: &'a InputMap,
    water: &'a InputMap,
    light: &'a InputMap,
    temperature: &'a InputMap,
    humidity: &'a InputMap,
}

impl InputSet<'_> {
    fn filter(input_maps: &Vec<InputMap>, category: Category) -> MyResult<&InputMap> {
        Ok(input_maps
            .iter()
            .find(|input_map| input_map.from.eq(&category))
            .ok_or("seed?")?)
    }
    fn of(input_maps: &Vec<InputMap>) -> MyResult<InputSet> {
        let seed = InputSet::filter(input_maps, Category::Seed)?;
        let soil = InputSet::filter(input_maps, Category::Soil)?;
        let fertilizer = InputSet::filter(input_maps, Category::Fertilizer)?;
        let water = InputSet::filter(input_maps, Category::Water)?;
        let light = InputSet::filter(input_maps, Category::Light)?;
        let temperature = InputSet::filter(input_maps, Category::Temperature)?;
        let humidity = InputSet::filter(input_maps, Category::Humidity)?;

        let tr_ranges = seed.tr_ranges.join(&soil.tr_ranges);
        println!(
            "\n\nseed {:?} \njoin {:?} \n= {:?}",
            seed.tr_ranges, soil.tr_ranges, tr_ranges
        );

        Ok(InputSet {
            seed,
            soil,
            fertilizer,
            water,
            light,
            temperature,
            humidity,
        })
    }
    fn tr(&self, seed: u64) -> u64 {
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

    fn tr_ranges(&self, seed_ranges: &Vec<&Range>) -> Vec<Range> {
        let soil = self.seed.tr_ranges(seed_ranges);
        vec![]
    }
}

#[derive(Debug)]
struct TranslationRanges {
    ranges: Vec<TranslationRange>,
}

impl TranslationRanges {
    fn intersect(&self, other: &TranslationRange) -> Vec<TranslationRange> {
        self.ranges
            .iter()
            .filter_map(|that| that.join(&other))
            .collect()
    }
    fn join(&self, other: &TranslationRanges) -> TranslationRanges {
        let ranges: Vec<TranslationRange> = other
            .ranges
            .iter()
            .map(|other| self.intersect(other))
            .collect::<Vec<Vec<TranslationRange>>>()
            .concat();
        TranslationRanges { ranges }
    }
    fn tr(&self, val: u64) -> Option<u64> {
        self.ranges.iter().find_map(|pair| pair.tr(val))
    }
}

#[derive(Debug)]
struct InputMap {
    from: Category,
    tr_ranges: TranslationRanges,
}

impl InputMap {
    fn read(lines: &mut impl Iterator<Item = io::Result<String>>) -> MyResult<Option<InputMap>> {
        if let Some(title_line) = lines.next() {
            let title_line = title_line?;
            let from_sep = title_line.find("-to-").ok_or("'-to-' ?")?;
            let from = &title_line[0..from_sep];
            let mut ranges: Vec<TranslationRange> = vec![];
            loop {
                if let Some(line) = lines.next() {
                    let line = line?;
                    if line.len() == 0 {
                        break;
                    }
                    let parts: Vec<&str> = line.splitn(3, ' ').collect();
                    ranges.push(TranslationRange::from(&parts)?);
                } else {
                    break;
                }
            }
            Ok(Some(InputMap {
                from: Category::of(from),
                tr_ranges: TranslationRanges { ranges },
            }))
        } else {
            Ok(None)
        }
    }
    fn tr(&self, val: u64) -> u64 {
        self.tr_ranges.tr(val).get_or_insert(val).to_owned()
    }
    fn tr_range(&self, range: &Range) -> Vec<Range> {
        let mut vec: Vec<Range> = self
            .tr_ranges
            .ranges
            .iter()
            .filter_map(|pair| pair.tr_range(range))
            .collect();
        if vec.len() == 0 {
            vec.push(Range::clone(range))
        }
        vec
    }

    fn tr_ranges(&self, ranges: &Vec<&Range>) -> Vec<Range> {
        ranges
            .iter()
            .map(|range| self.tr_range(range))
            .collect::<Vec<Vec<Range>>>()
            .concat()
    }
}

fn seeds(line: &str) -> MyResult<Vec<u64>> {
    let space = line.find(' ').ok_or("space?")?;
    let seed_str = &line[space + 1..];
    let seeds: Vec<u64> = seed_str
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
    let mut input_maps: Vec<InputMap> = vec![];
    loop {
        if let Some(map) = InputMap::read(&mut lines)? {
            input_maps.push(map);
        } else {
            break;
        }
    }
    let input_set = InputSet::of(&input_maps)?;
    let min = seeds
        .iter()
        .map(|val| input_set.tr(val.to_owned()))
        .min()
        .ok_or("min?")?;
    println!("day5p1 {}", min);
    Ok(())
}

fn seed_pairs(line: &str) -> MyResult<Vec<Range>> {
    let space = line.find(' ').ok_or("space?")?;
    let seed_str = &line[space + 1..];
    let (_, ranges) = seed_str.split(' ').map(|a| a.parse::<u64>()).try_fold(
        (None, vec![]),
        |(prev, vec), next| -> MyResult<(Option<u64>, Vec<Range>)> {
            match prev {
                Some(prev) => Ok((
                    None,
                    [vec, vec![Range::start_size(prev, next?).ok_or("size?")?]].concat(),
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
    let mut input_maps: Vec<InputMap> = vec![];
    loop {
        if let Some(map) = InputMap::read(&mut lines)? {
            input_maps.push(map);
        } else {
            break;
        }
    }
    let input_set = InputSet::of(&input_maps)?;
    // let min: Vec<Range> = seed_pairs
    //     .into_iter()
    //     .filter_map(|range| input_set.tr_range(&range))
    //     .collect();
    // println!("day5p2 {:?}", min);
    Ok(())
}
