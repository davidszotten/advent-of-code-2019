// use aoc2019::coor::Coor;
use aoc2019::cpu::Cpu;
use aoc2019::{dispatch, Result};
use rand::{thread_rng, Rng};
// use failure::bail;
// use std::collections::HashMap;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

/*

NOT C T
NOT A J
OR T J
AND D J

*/

fn part1(input: &str) -> Result<i32> {
    let mut cpu = Cpu::from_str(input);
    cpu.expect_ascii("Input instructions:\n")?;

    /*
       T: ~C
       J: (~A | ~C) & D
    */
    cpu.write_ascii(
        "\
NOT C T
NOT A J
OR T J
AND D J
",
    );
    cpu.write_ascii(
        "\
AND I J
NOT C T
AND H T
OR T J
AND D J
RUN\n",
    );
    // 213245
    cpu.expect_ascii("\nRunning...\n\n")?;
    cpu.read_ascii(true)?;
    dbg!(cpu.time_elapsed());
    Ok(0)
}
fn part2(input: &str) -> Result<i32> {
    let _available_walk = [
        "AND A J", "AND A T", "AND B J", "AND B T", "AND C J", "AND C T", "AND D J", "AND D T",
        "AND T J", "AND J T", "OR A J", "OR A T", "OR B J", "OR B T", "OR C J", "OR C T", "OR D J",
        "OR D T", "OR T J", "OR J T", "NOT A J", "NOT A T", "NOT B J", "NOT B T", "NOT C J",
        "NOT C T", "NOT D J", "NOT D T", "NOT T J", "NOT J T", "NOT J J", "NOT T T",
    ];

    let _available_run = [
        "AND A J", "AND A T", "AND B J", "AND B T", "AND C J", "AND C T", "AND D J", "AND D T",
        "AND E J", "AND E J", "AND F J", "AND F T", "AND G T", "AND G T", "AND H T", "AND H T",
        "AND I J", "AND I J", "AND T J", "AND J T", "OR A J", "OR A T", "OR B J", "OR B T",
        "OR C J", "OR C T", "OR D J", "OR D T", "OR T J", "OR J T", "OR E J", "OR E J", "OR F J",
        "OR F T", "OR G T", "OR G T", "OR H T", "OR H T", "OR I J", "OR I J", "NOT A J", "NOT A T",
        "NOT B J", "NOT B T", "NOT C J", "NOT C T", "NOT D J", "NOT D T", "NOT T J", "NOT J T",
        "NOT J J", "NOT T T", "NOT E J", "NOT E J", "NOT F J", "NOT F T", "NOT G T", "NOT G T",
        "NOT H T", "NOT H T", "NOT I J", "NOT I J",
    ];

    let available = _available_run;
    dbg!(available.len());

    let mut best_inst = "".to_string();
    let mut best_time = 0;

    let mut rng = thread_rng();
    let mut cpu = Cpu::from_str(input);
    cpu.expect_ascii("Input instructions:\n")?;

    cpu.write_ascii(
        "\
NOT C T
NOT A J
OR T J
AND D J
",
    );

    for _ in 0..100_000 {
        let mut cpu = cpu.clone();
        let mut inst = "".to_string();

        let count = rng.gen_range(1, 16 - 4);

        for _ in 0..count {
            let idx = rng.gen_range(0, available.len());
            cpu.write_ascii(available[idx]);
            /* println!("{}", available[idx]); */
            inst = format!("{}{}\n", inst, available[idx]);
            cpu.write_ascii("\n");
        }
        cpu.write_ascii("RUN\n");
        cpu.expect_ascii("\nRunning...\n\n")?;
        match cpu.expect_ascii("\nDidn't make it") {
            Ok(_) => {}
            Err(_) => {
                println!("{}", inst);
                return Ok(-1);
            }
        }
        cpu.read_ascii(false)?;
        /* dbg!(cpu.time_elapsed()); */
        if cpu.time_elapsed() > best_time {
            best_time = cpu.time_elapsed();
            best_inst = inst
        }
    }
    println!("{}", best_time);
    println!("{}", best_inst);
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1("")?, 0);
        Ok(())
    }
}
