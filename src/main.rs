mod cli;
mod commands;
mod io;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use commands::{extend, get_fasta, intersect, merge, name_map, random, sample, sort};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Sort { input, output } => sort(input, output)?,
        Command::Merge {
            input,
            output,
            sorted,
        } => merge(input, output, sorted)?,
        Command::Intersect {
            a,
            b,
            output,
            fraction_query,
            fraction_target,
            reciprocal,
            either,
        } => intersect(
            a,
            b,
            output,
            fraction_query,
            fraction_target,
            reciprocal,
            either,
        )?,
        Command::GetFasta {
            bed,
            fasta,
            output,
            map,
            threads,
        } => get_fasta(bed, &fasta, output, map, threads)?,
        Command::NameMap { input, output, map } => name_map(input, output, map)?,
        Command::Extend {
            input,
            output,
            both,
            left,
            right,
            genome,
        } => extend(input, output, both, left, right, genome)?,
        Command::Random {
            n_intervals,
            l_intervals,
            n_chr,
            max_chr_len,
            seed,
            output,
        } => random(n_intervals, l_intervals, n_chr, max_chr_len, seed, output)?,
        Command::Sample {
            input,
            output,
            number,
            fraction,
            seed,
        } => sample(input, output, number, fraction, seed)?,
    }
    Ok(())
}
