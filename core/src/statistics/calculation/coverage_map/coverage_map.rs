use std::collections::LinkedList;
use std::num::NonZeroU32;
use serde_derive::{Serialize, Deserialize};
use std::cmp::{max, min};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CoverageMap {
	#[serde(rename = "s")]
	pub start: u32,
	#[serde(rename = "end")]
	pub end: NonZeroU32,
	#[serde(rename = "ent")]
	pub entries: LinkedList<CoverageEntry>
}

impl CoverageMap {
	pub fn new(start: u32, end: NonZeroU32) -> Self {
		CoverageMap {
			start,
			end,
			entries: LinkedList::new()
		}
	}

	pub fn add_coverage(&mut self, start: u32, end: u32) {
		//An Entry with negative Length contributes no coverage
		if end < start {return;}

		//An Entry outside the Coverage Map area contributes no coverage
		if start < self.start && end < u32::from(self.end) {return;}
		if start > u32::from(self.end) && end > u32::from(self.end) {return;}

		//Clip the Bounds of the Entry to inside the coverage area
		let start = max(self.start, start);
		let end = min(u32::from(self.end), end);

		let mut cursor = self.entries.cursor_front_mut();

		let mut current_start = start;

		loop {
			//If we have reached the end of the list (or the list is empty) we insert the rest of our entry at the end
			if cursor.current().is_none() {
				//Inserting before is ok even if the List is empty
				cursor.insert_before(CoverageEntry {
					start: current_start,
					end,
					coverage: 1
				});
				return;
			}

			let current = cursor.current().unwrap();

			//If we have some coverage in front of the existing entry
			let has_in_front = current_start < current.start;

			//If we have some coverage behind the existing entry
			let has_behind = current.end < end;

			//If the new entry completely covers the existing entry
			let has_complete_coverage = current_start <= current.start && current.end <= end;

			//If the new entry is completely covered by the existing entry
			let has_complete_inclusion = current.start <= current_start && end <= current.end;

			//If the new entry begins outside the existing entry and ends somewhere in it
			let has_partial_left_coverage = current_start <= current.start && current.start <= end && end < current.end;

			//If the new entry begins inside the existing entry and ends outside it
			let has_partial_right_coverage = current.start < current_start && current_start <= current.end;

			let entry_start = current.start;
			let entry_end = current.end;
			let entry_coverage = current.coverage;

			drop(current);

			if has_in_front {
				cursor.insert_before(CoverageEntry {
					start: current_start,
					end: entry_start - 1,
					coverage: 1
				});
				current_start = entry_start;
			}

			if has_partial_left_coverage {
				cursor.insert_after(CoverageEntry {
					start: end + 1,
					end: entry_end,
					coverage: entry_coverage
				});

				let current = cursor.current().unwrap();
				current.coverage = entry_coverage + 1;
				current.end = end;

				return;
			}

			if has_complete_inclusion {
				cursor.insert_before(CoverageEntry {
					start: entry_start,
					end: current_start - 1,
					coverage: entry_coverage
				});
				cursor.insert_after(CoverageEntry {
					start: end + 1,
					end: entry_end,
					coverage: entry_coverage
				});

				let current = cursor.current().unwrap();
				current.coverage = entry_coverage + 1;
				current.start = current_start;
				current.end = end;

				return;
			}

			if has_partial_right_coverage {
				cursor.insert_before(CoverageEntry {
					start: entry_start,
					end: current_start - 1,
					coverage: entry_coverage
				});

				let current = cursor.current().unwrap();
				current.coverage = entry_coverage + 1;
				current.start = current_start
			}

			if has_complete_coverage {
				let current = cursor.current().unwrap();

				current.coverage = entry_coverage + 1;
			}

			if !has_behind {
				return;
			}

			if current_start <= entry_end {
				current_start = entry_end + 1;
			}
			cursor.move_next();
		}
	}

	pub fn merge(mut lhs: CoverageMap, mut rhs: CoverageMap) -> CoverageMap {
		let map_end = max(lhs.end, rhs.end);
		let map_start = min(lhs.start, rhs.start);

		let mut new_map = CoverageMap::new(map_start, map_end);

		if lhs.entries.is_empty() && rhs.entries.is_empty() {return new_map;}
		if lhs.entries.is_empty() {return rhs.clone();}
		if rhs.entries.is_empty() {return lhs.clone();}

		let mut lhs_cursor = lhs.entries.cursor_front_mut();
		let mut rhs_cursor = rhs.entries.cursor_front_mut();

		loop {
			let lhs_current = lhs_cursor.current();
			let rhs_current = rhs_cursor.current();

			if lhs_current.is_none() && rhs_current.is_none() {
				break;
			}
			else if lhs_current.is_none() {
				let rhs_entry = rhs_current.unwrap();
				new_map.entries.push_back(*rhs_entry);
				rhs_cursor.move_next();
				continue;
			}
			else if rhs_current.is_none() {
				let lhs_entry = lhs_current.unwrap();
				new_map.entries.push_back(*lhs_entry);
				lhs_cursor.move_next();
				continue;
			}

			let lhs_current = lhs_current.unwrap();
			let rhs_current = rhs_current.unwrap();

			if lhs_current.start < rhs_current.start {
				if lhs_current.end < rhs_current.start {
					new_map.entries.push_back(CoverageEntry {
						start: lhs_current.start,
						end: lhs_current.end,
						coverage: lhs_current.coverage
					});
					lhs_cursor.move_next();
					continue;
				}
				else {
					new_map.entries.push_back(CoverageEntry {
						start: lhs_current.start,
						end: rhs_current.start - 1,
						coverage: lhs_current.coverage
					});

					if lhs_current.end < rhs_current.end {
						new_map.entries.push_back(CoverageEntry {
							start: rhs_current.start,
							end: lhs_current.end,
							coverage: lhs_current.coverage + rhs_current.coverage
						});
						rhs_current.start = lhs_current.end + 1;
						lhs_cursor.move_next();
						continue;
					}
					else if lhs_current.end == rhs_current.end {
						new_map.entries.push_back(CoverageEntry {
							start: rhs_current.start,
							end: rhs_current.end,
							coverage: lhs_current.coverage + rhs_current.coverage
						});
						lhs_cursor.move_next();
						rhs_cursor.move_next();
						continue;
					}
					else {
						new_map.entries.push_back(CoverageEntry {
							start: rhs_current.start,
							end: rhs_current.end,
							coverage: lhs_current.coverage + rhs_current.coverage
						});
						lhs_current.start = rhs_current.end + 1;
						rhs_cursor.move_next();
						continue;
					}
				}
			}
			else if lhs_current.start == rhs_current.start {
				if lhs_current.end < rhs_current.end {
					new_map.entries.push_back(CoverageEntry {
						start: lhs_current.start,
						end: lhs_current.end,
						coverage: lhs_current.coverage + rhs_current.coverage
					});
					rhs_current.start = lhs_current.end + 1;
					lhs_cursor.move_next();
					continue;
				}
				else if lhs_current.end == rhs_current.end {
					new_map.entries.push_back(CoverageEntry {
						start: lhs_current.start,
						end: lhs_current.end,
						coverage: lhs_current.coverage + rhs_current.coverage
					});
					lhs_cursor.move_next();
					rhs_cursor.move_next();
					continue;
				}
				else {
					new_map.entries.push_back(CoverageEntry {
						start: lhs_current.start,
						end: rhs_current.end,
						coverage: lhs_current.coverage + rhs_current.coverage
					});
					lhs_current.start = rhs_current.end + 1;
					rhs_cursor.move_next();
					continue;
				}
			}
			else {
				if rhs_current.end < lhs_current.start {
					new_map.entries.push_back(CoverageEntry {
						start: rhs_current.start,
						end: rhs_current.end,
						coverage: rhs_current.coverage
					});
					rhs_cursor.move_next();
					continue;
				}
				else {
					new_map.entries.push_back(CoverageEntry {
						start: rhs_current.start,
						end: lhs_current.start - 1,
						coverage: rhs_current.coverage
					});

					if rhs_current.end < lhs_current.end {
						new_map.entries.push_back(CoverageEntry {
							start: lhs_current.start,
							end: rhs_current.end,
							coverage: lhs_current.coverage + rhs_current.coverage
						});
						lhs_current.start = rhs_current.end + 1;
						rhs_cursor.move_next();
						continue;
					}
					else if lhs_current.end == rhs_current.end {
						new_map.entries.push_back(CoverageEntry {
							start: lhs_current.start,
							end: lhs_current.end,
							coverage: lhs_current.coverage + rhs_current.coverage
						});
						lhs_cursor.move_next();
						rhs_cursor.move_next();
						continue;
					}
					else {
						new_map.entries.push_back(CoverageEntry {
							start: lhs_current.start,
							end: lhs_current.end,
							coverage: lhs_current.coverage + rhs_current.coverage
						});
						rhs_current.start = lhs_current.end + 1;
						lhs_cursor.move_next();
						continue;
					}
				}
			}
		}

		return new_map;
	}

	pub fn get_total_covered_length(&self) -> u32 {
		self.entries
			.iter()
			.map(|entry| ((entry.end - entry.start) + 1))
			.sum()
	}
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct CoverageEntry {
	#[serde(rename = "s")]
	pub start: u32,
	#[serde(rename = "e")]
	pub end: u32,
	#[serde(rename = "c")]
	pub coverage: u32
}

#[cfg(test)]
mod insert_tests {
	use crate::statistics::calculation::coverage_map::coverage_map::CoverageMap;
	use std::num::NonZeroU32;

	#[test]
	fn disjointed_entries_smaller_first() {
		let mut coverage_map = CoverageMap::new(0,NonZeroU32::new(200).unwrap());

		let entry_a_start = 0u32;
		let entry_a_end = 100u32;
		let entry_b_start = 101u32;
		let entry_b_end = 200u32;

		coverage_map.add_coverage(entry_a_start, entry_a_end);
		coverage_map.add_coverage(entry_b_start, entry_b_end);

		assert_eq!(coverage_map.entries.len(), 2);

		let first = coverage_map.entries.front().unwrap();
		let last = coverage_map.entries.back().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_a_end);

		assert_eq!(last.start, entry_b_start);
		assert_eq!(last.end, entry_b_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 201);
	}

	#[test]
	fn disjointed_entries_smaller_first_non_total_coverage() {
		let mut coverage_map = CoverageMap::new(0,NonZeroU32::new(200).unwrap());

		let entry_a_start = 50u32;
		let entry_a_end = 100u32;
		let entry_b_start = 101u32;
		let entry_b_end = 150u32;

		coverage_map.add_coverage(entry_a_start, entry_a_end);
		coverage_map.add_coverage(entry_b_start, entry_b_end);

		assert_eq!(coverage_map.entries.len(), 2);

		let first = coverage_map.entries.front().unwrap();
		let last = coverage_map.entries.back().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_a_end);

		assert_eq!(last.start, entry_b_start);
		assert_eq!(last.end, entry_b_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 101);
	}

	#[test]
	fn coverage_for_non_adjoint_entries() {
		let mut coverage_map = CoverageMap::new(0,NonZeroU32::new(200).unwrap());

		let entry_a_start = 25u32;
		let entry_a_end = 75u32;
		let entry_b_start = 125u32;
		let entry_b_end = 175u32;

		coverage_map.add_coverage(entry_a_start, entry_a_end);
		coverage_map.add_coverage(entry_b_start, entry_b_end);

		assert_eq!(coverage_map.entries.len(), 2);

		let first = coverage_map.entries.front().unwrap();
		let last = coverage_map.entries.back().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_a_end);

		assert_eq!(last.start, entry_b_start);
		assert_eq!(last.end, entry_b_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 102);
	}

	#[test]
	fn disjointed_entries_bigger_first() {
		let mut coverage_map = CoverageMap::new(0,NonZeroU32::new(200).unwrap());

		let entry_a_start = 0u32;
		let entry_a_end = 100u32;
		let entry_b_start = 101u32;
		let entry_b_end = 200u32;

		coverage_map.add_coverage(entry_b_start, entry_b_end);
		coverage_map.add_coverage(entry_a_start, entry_a_end);

		assert_eq!(coverage_map.entries.len(), 2);

		let first = coverage_map.entries.front().unwrap();
		let last = coverage_map.entries.back().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_a_end);

		assert_eq!(last.start, entry_b_start);
		assert_eq!(last.end, entry_b_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 201);
	}

	#[test]
	fn lower_bounds_clipping() {
		let mut coverage_map = CoverageMap::new(100,NonZeroU32::new(200).unwrap());

		let entry_start = 0u32;
		let entry_end = 200u32;

		coverage_map.add_coverage(entry_start, entry_end);

		assert_eq!(coverage_map.entries.len(), 1);

		let entry = coverage_map.entries.front().unwrap();

		assert_eq!(entry.start, 100);
		assert_eq!(entry.end, 200);

		assert_eq!(entry.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 101);
	}

	#[test]
	fn upper_bounds_clipping() {
		let mut coverage_map = CoverageMap::new(100,NonZeroU32::new(200).unwrap());

		let entry_start = 100u32;
		let entry_end = 300u32;

		coverage_map.add_coverage(entry_start, entry_end);

		assert_eq!(coverage_map.entries.len(), 1);

		let entry = coverage_map.entries.front().unwrap();

		assert_eq!(entry.start, 100);
		assert_eq!(entry.end, 200);

		assert_eq!(entry.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 101);
	}

	#[test]
	fn out_of_coverage_area_lower() {
		let mut coverage_map = CoverageMap::new(100,NonZeroU32::new(200).unwrap());

		let entry_start = 0u32;
		let entry_end = 99u32;

		coverage_map.add_coverage(entry_start, entry_end);

		assert_eq!(coverage_map.entries.len(), 0);

		assert_eq!(coverage_map.get_total_covered_length(), 0);
	}

	#[test]
	fn out_of_coverage_area_upper() {
		let mut coverage_map = CoverageMap::new(100,NonZeroU32::new(200).unwrap());

		let entry_start = 201u32;
		let entry_end = 300u32;

		coverage_map.add_coverage(entry_start, entry_end);

		assert_eq!(coverage_map.entries.len(), 0);

		assert_eq!(coverage_map.get_total_covered_length(), 0);
	}

	#[test]
	fn overlapping_entries_smaller_first() {
		let mut coverage_map = CoverageMap::new(0,NonZeroU32::new(200).unwrap());

		let entry_a_start = 0u32;
		let entry_a_end = 125u32;
		let entry_b_start = 75u32;
		let entry_b_end = 200u32;

		coverage_map.add_coverage(entry_a_start, entry_a_end);
		coverage_map.add_coverage(entry_b_start, entry_b_end);

		assert_eq!(coverage_map.entries.len(), 3);

		let mut entry_iter = coverage_map.entries.iter();

		let first = entry_iter.next().unwrap();
		let middle = entry_iter.next().unwrap();
		let last = entry_iter.next().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_b_start - 1);

		assert_eq!(middle.start, entry_b_start);
		assert_eq!(middle.end, entry_a_end);

		assert_eq!(last.start, entry_a_end + 1);
		assert_eq!(last.end, entry_b_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(middle.coverage, 2);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 201);
	}

	#[test]
	fn overlapping_entries_bigger_first() {
		let mut coverage_map = CoverageMap::new(0,NonZeroU32::new(200).unwrap());

		let entry_a_start = 0u32;
		let entry_a_end = 125u32;
		let entry_b_start = 75u32;
		let entry_b_end = 200u32;

		coverage_map.add_coverage(entry_b_start, entry_b_end);
		coverage_map.add_coverage(entry_a_start, entry_a_end);

		assert_eq!(coverage_map.entries.len(), 3);

		let mut entry_iter = coverage_map.entries.iter();

		let first = entry_iter.next().unwrap();
		let middle = entry_iter.next().unwrap();
		let last = entry_iter.next().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_b_start - 1);

		assert_eq!(middle.start, entry_b_start);
		assert_eq!(middle.end, entry_a_end);

		assert_eq!(last.start, entry_a_end + 1);
		assert_eq!(last.end, entry_b_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(middle.coverage, 2);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 201);
	}

	#[test]
	fn total_inclusion() {
		let mut coverage_map = CoverageMap::new(0,NonZeroU32::new(200).unwrap());

		let entry_a_start = 0u32;
		let entry_a_end = 200u32;
		let entry_b_start = 75u32;
		let entry_b_end = 125u32;

		coverage_map.add_coverage(entry_a_start, entry_a_end);
		coverage_map.add_coverage(entry_b_start, entry_b_end);

		assert_eq!(coverage_map.entries.len(), 3);

		let mut entry_iter = coverage_map.entries.iter();

		let first = entry_iter.next().unwrap();
		let middle = entry_iter.next().unwrap();
		let last = entry_iter.next().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_b_start - 1);

		assert_eq!(middle.start, entry_b_start);
		assert_eq!(middle.end, entry_b_end);

		assert_eq!(last.start, entry_b_end + 1);
		assert_eq!(last.end, entry_a_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(middle.coverage, 2);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 201);
	}

	#[test]
	fn total_coverage() {
		let mut coverage_map = CoverageMap::new(0,NonZeroU32::new(200).unwrap());

		let entry_a_start = 0u32;
		let entry_a_end = 200u32;
		let entry_b_start = 75u32;
		let entry_b_end = 125u32;

		coverage_map.add_coverage(entry_b_start, entry_b_end);
		coverage_map.add_coverage(entry_a_start, entry_a_end);

		assert_eq!(coverage_map.entries.len(), 3);

		let mut entry_iter = coverage_map.entries.iter();

		let first = entry_iter.next().unwrap();
		let middle = entry_iter.next().unwrap();
		let last = entry_iter.next().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_b_start - 1);

		assert_eq!(middle.start, entry_b_start);
		assert_eq!(middle.end, entry_b_end);

		assert_eq!(last.start, entry_b_end + 1);
		assert_eq!(last.end, entry_a_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(middle.coverage, 2);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 201);
	}
}

#[cfg(test)]
mod merge_tests {
	use crate::statistics::calculation::coverage_map::coverage_map::CoverageMap;
	use std::num::NonZeroU32;

	#[test]
	fn lower_bound_smaller_first() {
		let coverage_map_a = CoverageMap::new(0,NonZeroU32::new(200).unwrap());
		let coverage_map_b = CoverageMap::new(100,NonZeroU32::new(200).unwrap());

		let coverage_map = CoverageMap::merge(coverage_map_a, coverage_map_b);

		assert_eq!(coverage_map.start, 0);

		assert_eq!(coverage_map.get_total_covered_length(), 0);
	}

	#[test]
	fn lower_bound_bigger_first() {
		let coverage_map_a = CoverageMap::new(0,NonZeroU32::new(200).unwrap());
		let coverage_map_b = CoverageMap::new(100,NonZeroU32::new(200).unwrap());

		let coverage_map = CoverageMap::merge(coverage_map_b, coverage_map_a);

		assert_eq!(coverage_map.start, 0);

		assert_eq!(coverage_map.get_total_covered_length(), 0);
	}

	#[test]
	fn upper_bound_smaller_first() {
		let coverage_map_a = CoverageMap::new(0,NonZeroU32::new(100).unwrap());
		let coverage_map_b = CoverageMap::new(0,NonZeroU32::new(200).unwrap());

		let coverage_map = CoverageMap::merge(coverage_map_a, coverage_map_b);

		assert_eq!(u32::from(coverage_map.end), 200);

		assert_eq!(coverage_map.get_total_covered_length(), 0);
	}

	#[test]
	fn upper_bound_bigger_first() {
		let coverage_map_a = CoverageMap::new(0,NonZeroU32::new(100).unwrap());
		let coverage_map_b = CoverageMap::new(100,NonZeroU32::new(200).unwrap());

		let coverage_map = CoverageMap::merge(coverage_map_b, coverage_map_a);

		assert_eq!(u32::from(coverage_map.end), 200);

		assert_eq!(coverage_map.get_total_covered_length(), 0);
	}

	#[test]
	fn disjointed_bounds_smaller_first() {
		let coverage_map_a = CoverageMap::new(0,NonZeroU32::new(100).unwrap());
		let coverage_map_b = CoverageMap::new(101,NonZeroU32::new(200).unwrap());

		let coverage_map = CoverageMap::merge(coverage_map_a, coverage_map_b);

		assert_eq!(coverage_map.start, 0);
		assert_eq!(u32::from(coverage_map.end), 200);

		assert_eq!(coverage_map.get_total_covered_length(), 0);
	}

	#[test]
	fn disjointed_bounds_bigger_first() {
		let coverage_map_a = CoverageMap::new(0,NonZeroU32::new(100).unwrap());
		let coverage_map_b = CoverageMap::new(101,NonZeroU32::new(200).unwrap());

		let coverage_map = CoverageMap::merge(coverage_map_b, coverage_map_a);

		assert_eq!(coverage_map.start, 0);
		assert_eq!(u32::from(coverage_map.end), 200);

		assert_eq!(coverage_map.get_total_covered_length(), 0);
	}


	#[test]
	fn disjointed_entries_smaller_first() {
		let mut coverage_map_a = CoverageMap::new(0,NonZeroU32::new(200).unwrap());
		let mut coverage_map_b = CoverageMap::new(0,NonZeroU32::new(200).unwrap());

		let entry_a_start = 0u32;
		let entry_a_end = 100u32;
		let entry_b_start = 101u32;
		let entry_b_end = 200u32;

		coverage_map_a.add_coverage(entry_a_start, entry_a_end);
		coverage_map_b.add_coverage(entry_b_start, entry_b_end);

		let coverage_map = CoverageMap::merge(coverage_map_a, coverage_map_b);

		assert_eq!(coverage_map.entries.len(), 2);

		let first = coverage_map.entries.front().unwrap();
		let last = coverage_map.entries.back().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_a_end);

		assert_eq!(last.start, entry_b_start);
		assert_eq!(last.end, entry_b_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 201);
	}

	#[test]
	fn disjointed_entries_smaller_first_non_total_coverage() {
		let mut coverage_map_a = CoverageMap::new(0,NonZeroU32::new(200).unwrap());
		let mut coverage_map_b = CoverageMap::new(0,NonZeroU32::new(200).unwrap());

		let entry_a_start = 50u32;
		let entry_a_end = 100u32;
		let entry_b_start = 101u32;
		let entry_b_end = 150u32;

		coverage_map_a.add_coverage(entry_a_start, entry_a_end);
		coverage_map_b.add_coverage(entry_b_start, entry_b_end);

		let coverage_map = CoverageMap::merge(coverage_map_a, coverage_map_b);

		assert_eq!(coverage_map.entries.len(), 2);

		let first = coverage_map.entries.front().unwrap();
		let last = coverage_map.entries.back().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_a_end);

		assert_eq!(last.start, entry_b_start);
		assert_eq!(last.end, entry_b_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 101);
	}

	#[test]
	fn coverage_for_non_adjoint_entries() {
		let mut coverage_map_a = CoverageMap::new(0,NonZeroU32::new(200).unwrap());
		let mut coverage_map_b = CoverageMap::new(0,NonZeroU32::new(200).unwrap());

		let entry_a_start = 25u32;
		let entry_a_end = 75u32;
		let entry_b_start = 125u32;
		let entry_b_end = 175u32;

		coverage_map_a.add_coverage(entry_a_start, entry_a_end);
		coverage_map_b.add_coverage(entry_b_start, entry_b_end);

		let coverage_map = CoverageMap::merge(coverage_map_a, coverage_map_b);

		assert_eq!(coverage_map.entries.len(), 2);

		let first = coverage_map.entries.front().unwrap();
		let last = coverage_map.entries.back().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_a_end);

		assert_eq!(last.start, entry_b_start);
		assert_eq!(last.end, entry_b_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 102);
	}

	#[test]
	fn disjointed_entries_bigger_first() {
		let mut coverage_map_a = CoverageMap::new(0,NonZeroU32::new(200).unwrap());
		let mut coverage_map_b = CoverageMap::new(0,NonZeroU32::new(200).unwrap());

		let entry_a_start = 0u32;
		let entry_a_end = 100u32;
		let entry_b_start = 101u32;
		let entry_b_end = 200u32;

		coverage_map_a.add_coverage(entry_a_start, entry_a_end);
		coverage_map_b.add_coverage(entry_b_start, entry_b_end);

		let coverage_map = CoverageMap::merge(coverage_map_b, coverage_map_a);

		assert_eq!(coverage_map.entries.len(), 2);

		let first = coverage_map.entries.front().unwrap();
		let last = coverage_map.entries.back().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_a_end);

		assert_eq!(last.start, entry_b_start);
		assert_eq!(last.end, entry_b_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 201);
	}

	#[test]
	fn overlapping_entries_smaller_first() {
		let mut coverage_map_a = CoverageMap::new(0,NonZeroU32::new(200).unwrap());
		let mut coverage_map_b = CoverageMap::new(0,NonZeroU32::new(200).unwrap());

		let entry_a_start = 0u32;
		let entry_a_end = 125u32;
		let entry_b_start = 75u32;
		let entry_b_end = 200u32;

		coverage_map_a.add_coverage(entry_a_start, entry_a_end);
		coverage_map_b.add_coverage(entry_b_start, entry_b_end);

		let coverage_map = CoverageMap::merge(coverage_map_a, coverage_map_b);

		assert_eq!(coverage_map.entries.len(), 3);

		let mut entry_iter = coverage_map.entries.iter();

		let first = entry_iter.next().unwrap();
		let middle = entry_iter.next().unwrap();
		let last = entry_iter.next().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_b_start - 1);

		assert_eq!(middle.start, entry_b_start);
		assert_eq!(middle.end, entry_a_end);

		assert_eq!(last.start, entry_a_end + 1);
		assert_eq!(last.end, entry_b_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(middle.coverage, 2);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 201);
	}

	#[test]
	fn overlapping_entries_bigger_first() {
		let mut coverage_map_a = CoverageMap::new(0,NonZeroU32::new(200).unwrap());
		let mut coverage_map_b = CoverageMap::new(0,NonZeroU32::new(200).unwrap());

		let entry_a_start = 0u32;
		let entry_a_end = 125u32;
		let entry_b_start = 75u32;
		let entry_b_end = 200u32;

		coverage_map_a.add_coverage(entry_a_start, entry_a_end);
		coverage_map_b.add_coverage(entry_b_start, entry_b_end);

		let coverage_map = CoverageMap::merge(coverage_map_b, coverage_map_a);

		assert_eq!(coverage_map.entries.len(), 3);

		let mut entry_iter = coverage_map.entries.iter();

		let first = entry_iter.next().unwrap();
		let middle = entry_iter.next().unwrap();
		let last = entry_iter.next().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_b_start - 1);

		assert_eq!(middle.start, entry_b_start);
		assert_eq!(middle.end, entry_a_end);

		assert_eq!(last.start, entry_a_end + 1);
		assert_eq!(last.end, entry_b_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(middle.coverage, 2);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 201);
	}

	#[test]
	fn total_inclusion() {
		let mut coverage_map_a = CoverageMap::new(0,NonZeroU32::new(200).unwrap());
		let mut coverage_map_b = CoverageMap::new(0,NonZeroU32::new(200).unwrap());

		let entry_a_start = 0u32;
		let entry_a_end = 200u32;
		let entry_b_start = 75u32;
		let entry_b_end = 125u32;

		coverage_map_a.add_coverage(entry_a_start, entry_a_end);
		coverage_map_b.add_coverage(entry_b_start, entry_b_end);

		let coverage_map = CoverageMap::merge(coverage_map_a, coverage_map_b);

		assert_eq!(coverage_map.entries.len(), 3);

		let mut entry_iter = coverage_map.entries.iter();

		let first = entry_iter.next().unwrap();
		let middle = entry_iter.next().unwrap();
		let last = entry_iter.next().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_b_start - 1);

		assert_eq!(middle.start, entry_b_start);
		assert_eq!(middle.end, entry_b_end);

		assert_eq!(last.start, entry_b_end + 1);
		assert_eq!(last.end, entry_a_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(middle.coverage, 2);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 201);
	}

	#[test]
	fn total_coverage() {
		let mut coverage_map_a = CoverageMap::new(0,NonZeroU32::new(200).unwrap());
		let mut coverage_map_b = CoverageMap::new(0,NonZeroU32::new(200).unwrap());

		let entry_a_start = 0u32;
		let entry_a_end = 200u32;
		let entry_b_start = 75u32;
		let entry_b_end = 125u32;

		coverage_map_a.add_coverage(entry_a_start, entry_a_end);
		coverage_map_b.add_coverage(entry_b_start, entry_b_end);

		let coverage_map = CoverageMap::merge(coverage_map_b, coverage_map_a);

		assert_eq!(coverage_map.entries.len(), 3);

		let mut entry_iter = coverage_map.entries.iter();

		let first = entry_iter.next().unwrap();
		let middle = entry_iter.next().unwrap();
		let last = entry_iter.next().unwrap();

		assert_eq!(first.start, entry_a_start);
		assert_eq!(first.end, entry_b_start - 1);

		assert_eq!(middle.start, entry_b_start);
		assert_eq!(middle.end, entry_b_end);

		assert_eq!(last.start, entry_b_end + 1);
		assert_eq!(last.end, entry_a_end);

		assert_eq!(first.coverage, 1);
		assert_eq!(middle.coverage, 2);
		assert_eq!(last.coverage, 1);

		assert_eq!(coverage_map.get_total_covered_length(), 201);
	}
}