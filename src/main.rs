use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn codon_usage_bias(fasta_str: &str) -> HashMap<String, f64> {
    let mut codon_counts = HashMap::new();
    let mut total_count = 0;

    let mut lines = fasta_str.lines();
    while let Some(header) = lines.next() {
        // Skip the header line
        if !header.starts_with('>') {
            continue;
        }

        let mut sequence = String::new();
        while let Some(line) = lines.next() {
            if line.starts_with('>') {
                break;
            }
            sequence.push_str(line);
        }

        // Count the codons in the sequence
        for i in (0..sequence.len()).step_by(3) {
            let codon = &sequence[i..i + 3];
            *codon_counts.entry(codon.to_string()).or_insert(0) += 1;
            total_count += 1;
        }
    }

    // Compute the relative frequencies of each codon
    let mut codon_frequencies = HashMap::new();
    for (codon, count) in codon_counts {
        codon_frequencies.insert(codon, count as f64 / total_count as f64);
    }

    codon_frequencies
}

fn main() {
    let stdin = io::stdin();
    let fasta_str = stdin.lock().lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let codon_frequencies = codon_usage_bias(&fasta_str);
    for (codon, frequency) in codon_frequencies {
        println!("{}: {}", codon, frequency);
    }
}
