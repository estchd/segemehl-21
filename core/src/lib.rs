pub mod util;
pub mod header;
pub mod statistics;

#[cfg(test)]
mod test {
	#[test]
	pub fn test_bam_crate_flags() {
		let test_sam = "SRR058881.2752624 USI-EAS034_1_PE1_FC304J0AAXX:1:56:1759:682 length=64	99	adopted_3L	3814249	255	1M2I61M	=	3816684	2499	TGTCTGCATTAATTAAACGCGTTTTAATTGAGTAAAGCTGTTTTTATCTCACAACCACCACACA	...4...4......4.................................................	NM:i:4	MD:Z:0A45G15	NH:i:1	XI:i:0	XA:Z:P";

		let mut test_record = bam::Record::new();
		test_record.fill_from_sam(&test_sam, &bam::Header::new());

		assert!(test_record.flag().mate_is_mapped())
	}
}