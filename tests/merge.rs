#[cfg(test)]
mod testing {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use std::{fmt::Display, process::Command};

    fn build_expected_str<T: Display>(expected: &Vec<(T, u32, u32)>) -> String {
        expected
            .iter()
            .map(|(chr, start, end)| format!("{}\t{}\t{}\n", chr, start, end))
            .collect::<Vec<String>>()
            .join("")
    }

    fn build_expected_str_bed6<T: Display, S: Display, F: Display, C: Display>(
        expected: &Vec<(S, T, T, S, F, C)>,
    ) -> String {
        expected
            .iter()
            .map(|(chr, start, end, name, score, strand)| {
                format!(
                    "{}\t{}\t{}\t{}\t{:.1}\t{}\n",
                    chr, start, end, name, score, strand
                )
            })
            .collect::<Vec<String>>()
            .join("")
    }

    #[test]
    fn test_merge_sorted_bed3() -> Result<()> {
        let input = "tests/datasets/merge/sorted.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("merge").arg("-i").arg(input).output()?;

        let expected = vec![(1, 10, 45), (1, 100, 300), (2, 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_sorted_bed6() -> Result<()> {
        let input = "tests/datasets/merge/sorted.bed6";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("merge")
            .arg("-i")
            .arg(input)
            .arg("--format")
            .arg("bed6")
            .output()?;

        let expected = vec![
            (1, 10, 45, 0, 0.0, '+'),
            (1, 100, 300, 0, 0.0, '+'),
            (2, 105, 301, 0, 0.0, '+'),
        ];
        let expected_str = build_expected_str_bed6(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_sorted_skip_sort() -> Result<()> {
        let input = "tests/datasets/merge/sorted.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("merge").arg("-s").arg("-i").arg(input).output()?;

        let expected = vec![(1, 10, 45), (1, 100, 300), (2, 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_unsorted_bed3() -> Result<()> {
        let input = "tests/datasets/merge/unsorted.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("merge").arg("-i").arg(input).output()?;

        let expected = vec![(1, 10, 45), (1, 100, 300), (2, 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_unsorted_bed6() -> Result<()> {
        let input = "tests/datasets/merge/unsorted.bed6";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("merge")
            .arg("-i")
            .arg(input)
            .arg("--format")
            .arg("bed6")
            .output()?;

        let expected = vec![
            (1, 10, 45, 0, 0.0, '+'),
            (1, 100, 300, 0, 0.0, '+'),
            (2, 105, 301, 0, 0.0, '+'),
        ];
        let expected_str = build_expected_str_bed6(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_unsorted_named_bed3() -> Result<()> {
        let input = "tests/datasets/merge/unsorted_named.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("merge").arg("-N").arg("-i").arg(input).output()?;

        let expected = vec![("chr1", 10, 45), ("chr1", 100, 300), ("chr2", 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_unsorted_named_bed6() -> Result<()> {
        let input = "tests/datasets/merge/unsorted_named.bed6";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("merge")
            .arg("-N")
            .arg("-i")
            .arg(input)
            .arg("--format")
            .arg("bed6")
            .output()?;

        let expected = vec![
            ("chr1", 10, 45, "0", 0.0, '+'),
            ("chr1", 100, 300, "0", 0.0, '+'),
            ("chr2", 105, 301, "0", 0.0, '+'),
        ];
        let expected_str = build_expected_str_bed6(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_sorted_named_bed3() -> Result<()> {
        let input = "tests/datasets/merge/sorted_named.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("merge").arg("-N").arg("-i").arg(input).output()?;

        let expected = vec![("chr1", 10, 45), ("chr1", 100, 300), ("chr2", 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_sorted_named_bed6() -> Result<()> {
        let input = "tests/datasets/merge/sorted_named.bed6";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("merge")
            .arg("-N")
            .arg("-i")
            .arg(input)
            .arg("--format")
            .arg("bed6")
            .output()?;

        let expected = vec![
            ("chr1", 10, 45, "0", 0.0, '+'),
            ("chr1", 100, 300, "0", 0.0, '+'),
            ("chr2", 105, 301, "0", 0.0, '+'),
        ];
        let expected_str = build_expected_str_bed6(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_stream() -> Result<()> {
        let input = "tests/datasets/merge/sorted.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("merge").arg("-S").arg("-i").arg(input).output()?;

        let expected = vec![(1, 10, 45), (1, 100, 300), (2, 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }
}
