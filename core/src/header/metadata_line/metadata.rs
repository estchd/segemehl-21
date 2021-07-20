use crate::header::metadata_line::format_version::FormatVersion;
use crate::header::metadata_line::sorting_order::SortingOrder;
use crate::header::metadata_line::grouping_order::GroupingOrder;
use crate::header::metadata_line::sub_sorting_order::SubSortingOrder;
use bam::header::Tag;
use std::convert::TryFrom;

pub struct Metadata {
	pub format_version: FormatVersion,
	pub sorting_order: SortingOrder,
	pub grouping_order: GroupingOrder,
	pub sub_sorting_order: SubSortingOrder
}

impl Metadata {
	pub fn try_from_iter(iter: std::slice::Iter<'_, Tag>) -> Result<Metadata, ()> {
		let mut format_version: Option<FormatVersion> = None;
		let mut sorting_order: Option<SortingOrder> = None;
		let mut grouping_order: Option<GroupingOrder> = None;
		let mut sub_sorting_order: Option<SubSortingOrder> = None;

		for tag in iter {
			let tag = MetadataTag::try_from(tag)?;

			match tag {
				MetadataTag::FormatVersion(version) => {
					match format_version {
						None => {
							format_version = Some(version);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				MetadataTag::SortingOrder(order) => {
					match sorting_order {
						None => {
							sorting_order = Some(order);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				MetadataTag::GroupingOrder(order) => {
					match grouping_order {
						None => {
							grouping_order = Some(order);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
				MetadataTag::SubSortingOrder(order) => {
					match sub_sorting_order {
						None => {
							sub_sorting_order = Some(order);
						}
						Some(_) => {
							return Err(());
						}
					}
				}
			}
		}

		let format_version = format_version.ok_or(())?;
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

pub enum MetadataTag {
	FormatVersion(FormatVersion),
	SortingOrder(SortingOrder),
	GroupingOrder(GroupingOrder),
	SubSortingOrder(SubSortingOrder)
}

impl TryFrom<&Tag> for MetadataTag {
	type Error = ();

	fn try_from(value: &Tag) -> Result<Self, Self::Error> {
		let name_vec = value.name().to_vec();
		let name = String::from_utf8(name_vec).map_err(|_| ())?;

		match name.as_str() {
			"VN" => {
				let version = FormatVersion::try_from(value.value())?;
				Ok(Self::FormatVersion(version))
			},
			"SO" => {
				let order = SortingOrder::try_from(value.value())?;
				Ok(Self::SortingOrder(order))
			},
			"GO" => {
				let order = GroupingOrder::try_from(value.value())?;
				Ok(Self::GroupingOrder(order))
			},
			"SS" => {
				let order = SubSortingOrder::try_from(value.value())?;
				Ok(Self::SubSortingOrder(order))
			},
			_ => Err(())
		}
	}
}