#![feature(linked_list_cursors)]

pub mod util;
pub mod header;
pub mod statistics;


#[cfg(test)]
mod tests {
    use std::num::{NonZeroU32, NonZeroUsize};

    use bam::BamReader;
    use bam::header::{EntryType, HeaderEntry, HeaderLine, Tag};

    use crate::statistics::calculation::coverage_map::coverage_map::{CoverageEntry, CoverageMap};
    use crate::statistics::calculation::coverage_map::split_coverage_map::SplitCoverageMap;

    #[test]
    fn test_coverage_map_generation_with_first_chromosome_first_thousand_records() {
        let coverage_map = coverage_map_for_first_chromosome_in_sorted_file("./rep1.bam", 1000);

        let check_result = sanity_check_coverage_map(coverage_map);

        match check_result {
            SanityCheckResult::Ok => {}
            SanityCheckResult::EntryEndBeforeStart => {
                panic!("Entry End Before Start");
            }
            SanityCheckResult::EntryOverlap => {
                panic!("Entry Overlap");
            }
            SanityCheckResult::EntryMixup => {
                panic!("Entry Mixup");
            }
            SanityCheckResult::NoEntry => {
                panic!("No Entry")
            }
        }
    }

    enum SanityCheckResult {
        /// Coverage Map is Sane
        Ok,
        /// There is an Entry that has a Start Index that is greater than its End Index
        EntryEndBeforeStart,
        /// There are at least two entries whose Ranges have overlap (A_Start, B_Start, A_End, B_End)
        EntryOverlap,
        /// There are at least two entries where an entry is lower in its indices but comes later in the list
        EntryMixup,
        /// There are no entries in the Map
        NoEntry
    }

    fn coverage_map_for_first_chromosome_in_sorted_file(file: &str, max_records: usize) -> CoverageMap {
        let reader = BamReader::from_path(file, 8u16).unwrap();

        let header = reader.header();
        let refs: Vec<&HeaderEntry> = header.lines().filter_map(|item| match item {
            HeaderLine::Entry(entry) => Some(entry),
            HeaderLine::Comment(_) => None
        }).filter_map(|entry| match entry.entry_type() {
            EntryType::HeaderLine => None,
            EntryType::RefSequence => Some(entry),
            EntryType::ReadGroup => None,
            EntryType::Program => None,
        }).collect();

        if refs.is_empty() {panic!("No References!")}

        let first_ref = refs[0];

        let length_tags: Vec<&Tag> = first_ref.iter().filter_map(|tag|
            if tag.name() == "LN".as_bytes() {
                Some(tag)
            }
            else {
                None
            }
        ).collect();

        if length_tags.is_empty() {panic!("Reference has no Length!")}

        let length_tag = length_tags[0];

        let length: u32 = length_tag.value().parse().unwrap();

        let non_zero_length = NonZeroU32::new(length).unwrap();

        let mut coverage_map = SplitCoverageMap::new(0, non_zero_length, NonZeroUsize::new(1).unwrap());

        let mut record_count = 0usize;

        for record in reader.filter_map(|record| record.ok()) {
            if record.ref_id() != 0 {break;}
            if record.query_len() == 0 {continue;}

            let length = record.query_len();

            let start = record.start() as u32;

            let end = start + length;

            coverage_map.add_coverage(start, end);

            record_count = record_count + 1;
            if record_count >= max_records {break;}
        }

        return coverage_map.combine();
    }


    fn sanity_check_coverage_map(coverage_map: CoverageMap) -> SanityCheckResult {
        if coverage_map.entries.len() == 0 {
            return SanityCheckResult::NoEntry;
        }

        let mut last_entry: Option<CoverageEntry> = None;

        for entry in coverage_map.entries {
            if entry.start > entry.end {return SanityCheckResult::EntryEndBeforeStart;}

            match last_entry {
                None => {}
                Some(last_entry) => {
                    if last_entry.start >= entry.start {
                        if entry.end < last_entry.start {return SanityCheckResult::EntryMixup;}
                        else {return SanityCheckResult::EntryOverlap;}
                    }
                    else {
                        if last_entry.end >= entry.start {return SanityCheckResult::EntryOverlap;}
                    }
                }
            }

            last_entry = Some(entry);
        }

        SanityCheckResult::Ok
    }
}
