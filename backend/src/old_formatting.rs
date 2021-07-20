#![allow(dead_code)]
use bam::header::{TagName, Tag, HeaderEntry, EntryType, HeaderLine};
use bam::Header;

fn ident(lines: &mut Vec<String>) {
	for line in lines {
		*line = format!("\t{}", line);
	}
}

fn format_header(header: &Header) -> Vec<String> {
	let mut output = Vec::new();

	output.push("Header: {".to_string());

	for line in header.lines() {
		let mut formatted_line = format_header_line(line);
		ident(&mut formatted_line);
		output.append(&mut formatted_line);
	}

	output.push("}".to_string());
	return output;
}

fn format_header_line(line: &HeaderLine) -> Vec<String> {
	return match line {
		HeaderLine::Entry(entry) => format_header_entry(entry),
		HeaderLine::Comment(comment) => format_header_comment(comment)
	}
}

fn format_header_comment(comment: &String) -> Vec<String> {
	let mut output = Vec::<String>::new();
	output.push(format!("// {}",comment));
	return output;
}

fn format_header_entry(entry: &HeaderEntry) -> Vec<String> {
	let mut output = Vec::<String>::new();

	output.push("Entry: {".to_string());

	let mut formatted_entry_type = format_header_entry_type(entry.entry_type());
	ident(&mut formatted_entry_type);

	output.append(&mut formatted_entry_type);

	let mut tags_format = format_header_tags(entry);
	ident(&mut tags_format);

	output.append(&mut tags_format);

	output.push("}".to_string());
	return output;
}

fn format_header_entry_type(entry_type: EntryType) -> Vec<String> {
	let mut output = Vec::<String>::new();

	let type_format = match entry_type {
		EntryType::HeaderLine => "HeaderLine".to_string(),
		EntryType::RefSequence => "RefSequence".to_string(),
		EntryType::ReadGroup => "ReadGroup".to_string(),
		EntryType::Program => "Program".to_string(),
	};

	output.push(format!("Type: {}", type_format));

	return output;
}

fn format_header_tags(entry: &HeaderEntry) -> Vec<String> {
	let mut output = Vec::<String>::new();

	output.push(format!("Tag Count: {}", entry.len()));
	output.push("Tags: {".to_string());

	for tag in entry.iter() {
		let mut formatted_tag = format_header_tag(tag);

		ident(&mut formatted_tag);

		output.append(&mut formatted_tag);
	}

	output.push("}".to_string());

	return output;
}

fn format_header_tag(tag: &Tag) -> Vec<String> {
	let mut output = Vec::<String>::new();

	output.push("Tag: {".to_string());

	let mut formatted_tag_name = format_header_tag_name(tag.name());

	ident(&mut formatted_tag_name);

	output.append(&mut formatted_tag_name);

	let mut formatted_tag_value = format_header_tag_value(tag.value());

	ident(&mut formatted_tag_value);

	output.append(&mut formatted_tag_value);

	output.push("}".to_string());

	return output;
}

fn format_header_tag_name(tag_name: &TagName) -> Vec<String> {
	let mut output = Vec::<String>::new();

	output.push(format!("Name: {}",  String::from_utf8(tag_name.to_vec()).unwrap()));

	return output;
}

fn format_header_tag_value(tag_value: &str) -> Vec<String> {
	let mut output = Vec::<String>::new();

	output.push(format!("Value: {}", tag_value));

	return output;
}