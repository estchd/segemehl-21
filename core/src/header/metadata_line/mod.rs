use std::convert::TryFrom;

use bam::header::Tag;
use thiserror::Error;

use crate::header::metadata_line::format_version::{FormatVersion, FormatVersionError};
use crate::header::metadata_line::grouping_order::{GroupingOrder, GroupingOrderError};
use crate::header::metadata_line::sorting_order::{SortingOrder, SortingOrderError};
use crate::header::metadata_line::sub_sorting_order::{SubSortingOrder, SubSortingOrderError};
use std::string::FromUtf8Error;

pub mod format_version;
pub mod grouping_order;
pub mod sub_sorting_order;
pub mod sorting_order;

#[derive(Error, Debug)]
pub enum MetadataError {
	#[error("more than one format version tag, tag number: {tag_number}")]
	DuplicateFormatVersion {
		tag_number: usize
	},
	#[error("more than one sorting order tag, tag number: {tag_number}")]
	DuplicateSortingOrder {
		tag_number: usize
	},
	#[error("more than one grouping order tag, tag number: {tag_number}")]
	DuplicateGroupingOrder {
		tag_number: usize
	},
	#[error("more than one sub sorting order tag, tag number: {tag_number}")]
	DuplicateSubSortingOrder {
		tag_number: usize
	},
	#[error("no format version tag")]
	NoFormatVersion,
	#[error("tag with invalid tag name or content, tag number: {tag_number}")]
	InvalidTag {
		tag_number: usize,
		source: MetadataTagError
	}
}

pub struct Metadata {
	pub format_version: FormatVersion,
	pub sorting_order: SortingOrder,
	pub grouping_order: GroupingOrder,
	pub sub_sorting_order: SubSortingOrder
}

impl Metadata {
	pub fn try_from_iter(iter: std::slice::Iter<'_, Tag>) -> Result<Metadata, MetadataError> {
		let mut format_version: Option<FormatVersion> = None;
		let mut sorting_order: Option<SortingOrder> = None;
		let mut grouping_order: Option<GroupingOrder> = None;
		let mut sub_sorting_order: Option<SubSortingOrder> = None;

		for (tag_number, tag) in iter.enumerate() {
			let tag = MetadataTag::try_from(tag)
				.map_err(|err| MetadataError::InvalidTag {tag_number, source: err})?;

			match tag {
				MetadataTag::FormatVersion(version) => {
					match format_version {
						None => {
							format_version = Some(version);
						}
						Some(_) => {
							return Err(MetadataError::DuplicateFormatVersion {tag_number});
						}
					}
				}
				MetadataTag::SortingOrder(order) => {
					match sorting_order {
						None => {
							sorting_order = Some(order);
						}
						Some(_) => {
							return Err(MetadataError::DuplicateSortingOrder {tag_number});
						}
					}
				}
				MetadataTag::GroupingOrder(order) => {
					match grouping_order {
						None => {
							grouping_order = Some(order);
						}
						Some(_) => {
							return Err(MetadataError::DuplicateGroupingOrder {tag_number});
						}
					}
				}
				MetadataTag::SubSortingOrder(order) => {
					match sub_sorting_order {
						None => {
							sub_sorting_order = Some(order);
						}
						Some(_) => {
							return Err(MetadataError::DuplicateSubSortingOrder {tag_number});
						}
					}
				}
			}
		}

		let format_version = format_version.ok_or(MetadataError::NoFormatVersion)?;
		let sorting_order = sorting_order.unwrap_or(SortingOrder::Unknown);
		let grouping_order = grouping_order.unwrap_or(GroupingOrder::None);
		let sub_sorting_order = sub_sorting_order.unwrap_or(SubSortingOrder {
			sorting_order: crate::header::metadata_line::sub_sorting_order::SortingOrder::Unsorted,
			sub_sort: Vec::new()
		});

		Ok(Metadata {
			format_version,
			sorting_order,
			grouping_order,
			sub_sorting_order
		})
	}
}

#[derive(Error, Debug)]
pub enum MetadataTagError {
	#[error("tag name is not valid UTF-8, name values: {name_values:?}")]
	InvalidNameFormat {
		name_values: Vec<u8>,
		source: FromUtf8Error
	},
	#[error("tag name is not valid for a metadata line, name: {name}")]
	InvalidName {
		name: String
	},
	#[error("version number tag with invalid content, tag content: {content}")]
	InvalidVersionNumber {
		content: String,
		source: FormatVersionError
	},
	#[error("sorting order tag with invalid content, tag content: {content}")]
	InvalidSortingOrder {
		content: String,
		source: SortingOrderError
	},
	#[error("grouping order tag with invalid content, tag content: {content}")]
	InvalidGroupingOrder {
		content: String,
		source: GroupingOrderError
	},
	#[error("sub sorting order tag with invalid content, tag content: {content}")]
	InvalidSubSortingOrder {
		content: String,
		source: SubSortingOrderError
	}
}

pub enum MetadataTag {
	FormatVersion(FormatVersion),
	SortingOrder(SortingOrder),
	GroupingOrder(GroupingOrder),
	SubSortingOrder(SubSortingOrder)
}

impl TryFrom<&Tag> for MetadataTag {
	type Error = MetadataTagError;

	fn try_from(value: &Tag) -> Result<Self, Self::Error> {
		let name_vec = value.name().to_vec();
		let name = String::from_utf8(name_vec.clone())
			.map_err(|err| MetadataTagError::InvalidNameFormat {name_values: name_vec, source: err})?;

		match name.as_str() {
			"VN" => {
				let version = FormatVersion::try_from(value.value())
					.map_err(|err| MetadataTagError::InvalidVersionNumber {
						content: value.value().to_string(),
						source: err
					})?;
				Ok(Self::FormatVersion(version))
			},
			"SO" => {
				let order = SortingOrder::try_from(value.value())
					.map_err(|err| MetadataTagError::InvalidSortingOrder {
						content: value.value().to_string(),
						source: err
					})?;
				Ok(Self::SortingOrder(order))
			},
			"GO" => {
				let order = GroupingOrder::try_from(value.value())
					.map_err(|err| MetadataTagError::InvalidGroupingOrder {
						content: value.value().to_string(),
						source: err
					})?;
				Ok(Self::GroupingOrder(order))
			},
			"SS" => {
				let order = SubSortingOrder::try_from(value.value())
					.map_err(|err| MetadataTagError::InvalidSubSortingOrder {
						content: value.value().to_string(),
						source: err
					})?;
				Ok(Self::SubSortingOrder(order))
			},
			name => Err(MetadataTagError::InvalidName {name: name.to_string()})
		}
	}
}
