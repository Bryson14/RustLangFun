use regex::Regex;
use std::{ops::Range, vec};

#[derive(Debug, PartialEq)]
pub struct ConversionChart {
    pub name: String,
    pub conversions: Vec<Conversion>,
}

#[derive(Debug, PartialEq)]
pub struct Conversion {
    pub source: u64,
    pub destination: u64,
    pub range_len: u64,
}

#[derive(PartialEq, Debug)]
struct RangeMapping {
    converted_range: Option<Range<u64>>,
    unconverted_ranges: Vec<Range<u64>>,
}

impl ConversionChart {
    pub fn map_number_to_number(&self, source: &u64) -> u64 {
        let destination: u64 = *source;
        for conversion in &self.conversions {
            let possible = conversion.map_number_to_number(destination);
            // if there is a possible conversion, use it if lower than the current destination
            // TODO can there be multiple mappings for a single seed?
            if let Some(new_destination) = possible {
                return new_destination;
            }
        }
        destination
    }

    /// We can walk through each conversion and see if our input range is fully covered, needs to split between multiple conversions, or has to fall back onto the default 1 to 1 conversion.
    /// We have three scenarios:
    /// Where a single conversion covers the entire range of the input
    /// Where a single conversion covers one end of the input
    /// Where a single conversion covers a middle section of the input
    pub fn map_range_to_ranges(&self, range: Range<u64>) -> Vec<Range<u64>> {
        let mut unconverted_ranges: Vec<Range<u64>> = vec![range];
    
        for conversion in &self.conversions {
            let mut i = 0;
            while i < unconverted_ranges.len() {
                let unconverted_range = unconverted_ranges[i].clone();
                let mapping = conversion.map_range_to_ranges(unconverted_range);
    
                if let Some(converted_range) = mapping.converted_range {
                    unconverted_ranges.extend(mapping.unconverted_ranges);
                    unconverted_ranges.push(converted_range);
                } else {
                    // If the range couldn't be converted, keep it for the next iteration
                    unconverted_ranges.push(mapping.unconverted_ranges.into_iter().next().unwrap());
                }
    
                i += 1;
            }
            // Clear the unconverted_ranges and add the newly converted ranges
            unconverted_ranges.clear();
        }
    
        unconverted_ranges
    }
    
}

impl Conversion {
    pub fn map_number_to_number(&self, source: u64) -> Option<u64> {
        if source >= self.source && source < self.source + self.range_len {
            Some(source - self.source + self.destination)
        } else {
            None
        }
    }

    /// Maps a range to a vector of ranges. If the range is not within the source range, returns None.
    /// Example
    /// conversion is { source: 50, destination: 98, range_len: 2 }
    /// then map_range_to_ranges(50..52) returns Some(98..100)
    /// and map_range_to_ranges(51..53) returns Some(99..101)
    fn map_range_to_ranges(&self, range: Range<u64>) -> RangeMapping {
        // scenario 4: range is entirely outside the source range
        if (range.end < self.source) || (range.start > self.source + self.range_len) {
            return RangeMapping {
                converted_range: None,
                unconverted_ranges: vec![range],
            };
        } else if (range.start >= self.source) && (range.end <= self.source + self.range_len) {
            // scenario 1: range is entirely within the source range
            let converted_range = range.start - self.source + self.destination
                ..range.end - self.source + self.destination;
            RangeMapping {
                converted_range: Some(converted_range),
                unconverted_ranges: Vec::new(),
            }
        } else if range.start < self.source {
            if range.end <= self.source + self.range_len {
                // scenario 2.1: range starts before the source range, but ends within it
                let converted_range = self.destination..range.end - self.source + self.destination;
                let unconverted_range = range.start..self.source;
                RangeMapping {
                    converted_range: Some(converted_range),
                    unconverted_ranges: vec![unconverted_range],
                }
            } else {
                // scenario 3: range starts before the source range, and ends after source + len
                let converted_range = self.destination
                    ..self.destination + self.range_len;
                let left_unconverted_range = range.start..self.source;
                let right_unconverted_range = self.source + self.range_len..range.end;
                RangeMapping {
                    converted_range: Some(converted_range),
                    unconverted_ranges: vec![left_unconverted_range, right_unconverted_range],
                }
            }
        } else {
            // scenario 2.2: range starts after the source range, and ends after it
            let converted_range =
                self.destination..self.destination + (self.range_len - (range.start - self.source));
            let unconverted_range = self.source + self.range_len..range.end;
            RangeMapping {
                converted_range: Some(converted_range),
                unconverted_ranges: vec![unconverted_range],
            }
        }
    }
}

/// Reads in the input file and returns a vector of seeds and a vector of conversion charts.
/// ```
/// seeds: 79 14 55 13
///
/// seed-to-soil map:
/// 50 98 2
/// 52 50 48
/// ```
/// turns into ( [79,14,55,13], ConversionChart { name: "seed-to-soil", sourceStart: 50, destinationStart: 98, rangeLength: 2 } )
pub fn read_input(input: &str) -> (Vec<u64>, Vec<ConversionChart>) {
    let mut seeds: Vec<u64> = Vec::new();
    let mut charts: Vec<ConversionChart> = Vec::new();
    // a regex to match the conversion lines, which look like "50 98 2. Three numbers separated by spaces."
    let re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
    // a regex to match blank lines with optional newline characters
    let blank_re = Regex::new(r"^\s*\n?$").unwrap();

    // iterate over lines.

    for line in input.lines() {
        // seeds line
        if line.contains("seeds") {
            line.split(':')
                .nth(1)
                .unwrap()
                .split(' ')
                .filter(|s| !s.is_empty())
                .for_each(|s| seeds.push(s.parse::<u64>().unwrap()));
        } else if line.contains("map") {
            // start of a new chart
            let stripped = line.trim().split(' ').next().unwrap();
            charts.push(ConversionChart {
                name: stripped.to_string(),
                conversions: Vec::new(),
            });
        } else if re.is_match(line) {
            // conversion line contains numbers
            let caps = re.captures(line).unwrap();
            let source = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
            let destination = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let range_len = caps.get(3).unwrap().as_str().parse::<u64>().unwrap();
            let chart = charts.last_mut().unwrap();
            chart.conversions.push(Conversion {
                source,
                destination,
                range_len,
            });
        } else if blank_re.is_match(line) {
            // blank line, do nothing
        } else {
            panic!("Unrecognized line: {}", line);
        }
    }

    (seeds, charts)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_read_input_1() {
        let input_data = String::from("seeds: 79 14 55 13");
        let expected = vec![79, 14, 55, 13];
        let (seeds, charts) = read_input(&input_data);
        assert_eq!(seeds, expected);
        assert_eq!(charts.len(), 0);
    }

    #[test]
    fn test_read_input_2() {
        let input_data = String::from(
            "seeds: 79 14 55 13
        
        seed-to-soil map:
        50 98 2
        52 50 48",
        );
        let expected = vec![79, 14, 55, 13];
        let expected_chart = ConversionChart {
            name: String::from("seed-to-soil"),
            conversions: vec![
                Conversion {
                    source: 50,
                    destination: 98,
                    range_len: 2,
                },
                Conversion {
                    source: 52,
                    destination: 50,
                    range_len: 48,
                },
            ],
        };
        let (seeds, charts) = read_input(&input_data);
        assert_eq!(seeds, expected);
        assert_eq!(charts, vec![expected_chart]);
    }

    #[test]
    fn test_read_input_3() {
        let input_data = String::from(
            "seeds: 79 14 55 13
        
         seed-to-soil map:
         50 98 2
         52 50 48
        
         soil-to-fertilizer map:
         0 15 37
         37 52 2
         39 0 15",
        );
        let expected = vec![79, 14, 55, 13];
        let expected_chart = vec![
            ConversionChart {
                name: String::from("seed-to-soil"),
                conversions: vec![
                    Conversion {
                        source: 50,
                        destination: 98,
                        range_len: 2,
                    },
                    Conversion {
                        source: 52,
                        destination: 50,
                        range_len: 48,
                    },
                ],
            },
            ConversionChart {
                name: String::from("soil-to-fertilizer"),
                conversions: vec![
                    Conversion {
                        source: 0,
                        destination: 15,
                        range_len: 37,
                    },
                    Conversion {
                        source: 37,
                        destination: 52,
                        range_len: 2,
                    },
                    Conversion {
                        source: 39,
                        destination: 0,
                        range_len: 15,
                    },
                ],
            },
        ];

        let (seeds, charts) = read_input(&input_data);
        assert_eq!(seeds, expected);
        assert_eq!(charts, expected_chart);
    }

    #[test]
    fn test_conversion_map_number_to_number_1() {
        let conversion = Conversion {
            source: 50,
            destination: 98,
            range_len: 2,
        };
        assert_eq!(conversion.map_number_to_number(50), Some(98));
        assert_eq!(conversion.map_number_to_number(51), Some(99));
        assert_eq!(conversion.map_number_to_number(52), None);
    }

    #[test]
    fn test_map_range_to_ranges_1() {
        // the range is entirely within the source range
        let conversion = Conversion {
            source: 50,
            destination: 98,
            range_len: 2,
        };
        let expected = RangeMapping {
            converted_range: Some(98..100),
            unconverted_ranges: Vec::new(),
        };
        assert_eq!(conversion.map_range_to_ranges(50..52), expected);
    }

    #[test]
    fn test_map_range_to_ranges_2_1() {
        // the input range starts before the source range, but ends within it
        let conversion = Conversion {
            source: 50,
            destination: 98,
            range_len: 2,
        };
        let expected = RangeMapping {
            converted_range: Some(98..99),
            unconverted_ranges: vec![45..50],
        };
        assert_eq!(conversion.map_range_to_ranges(45..51), expected);
    }

    #[test]
    fn test_map_range_to_ranges_2_2() {
        // the input range starts after the source range, before then end, then ends after it
        let conversion = Conversion {
            source: 50,
            destination: 98,
            range_len: 2,
        };
        let expected = RangeMapping {
            converted_range: Some(98..100),
            unconverted_ranges: vec![52..55],
        };
        assert_eq!(conversion.map_range_to_ranges(50..55), expected);
    }

    #[test]
    fn test_map_range_to_ranges_3() {
        // the input range starts before the source range, and ends after source + len
        let conversion = Conversion {
            source: 50,
            destination: 98,
            range_len: 2,
        };
        let expected = RangeMapping {
            converted_range: Some(98..100),
            unconverted_ranges: vec![2..50, 52..60],
        };
        assert_eq!(conversion.map_range_to_ranges(2..60), expected);
    }

    #[test]
    fn test_map_range_to_ranges_4() {
        // the input range is entirely outside the source range
        let conversion = Conversion {
            source: 50,
            destination: 98,
            range_len: 2,
        };
        let expected = RangeMapping {
            converted_range: None,
            unconverted_ranges: vec![2..49],
        };
        assert_eq!(conversion.map_range_to_ranges(2..49), expected);
    }

    #[test]
    fn test_map_range_to_ranges() {
        let chart = ConversionChart{
            name: String::from("test"),
            conversions: vec![
                Conversion {
                    source: 50,
                    destination: 98,
                    range_len: 2,
                },
                Conversion {
                    source: 52,
                    destination: 37,
                    range_len: 2,
                },
            ],
        };
        let expected = vec![98..100, 37..39];
        assert_eq!(chart.map_range_to_ranges(50..54), expected);

    }
}
