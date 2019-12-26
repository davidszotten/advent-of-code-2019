use aoc2019::cpu::{Cpu, CpuState};
use aoc2019::{dispatch, Result};
use failure::err_msg;
use permutohedron::LexicalPermutation;
use std::io;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

fn part1(input: &str) -> Result<i32> {
    let mut cpu = Cpu::from_str(input);
    cpu.write_ascii(
        "\
north
west
take mug
west
take easter egg
east
east
south
south
take asterisk
south
west
north
take jam
south
east
north
east
take klein bottle
south
west
take tambourine
west
take cake
east
south
east
take polygon
north
",
    );
    /* let mut path = vec![]; */

    let mut items = vec![
        "polygon",
        "easter egg",
        "tambourine",
        "asterisk",
        "mug",
        "jam",
        "klein bottle",
        "cake",
    ];
    items.sort();

    for item in &items {
        cpu.write_ascii("drop ");
        cpu.write_ascii(item);
        cpu.write_ascii("\n");
    }
    cpu.read_ascii(false)?;
    let base = cpu.clone();

    'main: loop {
        for len in 1..items.len() {
            let mut cpu = base.clone();
            for item in &items[0..len] {
                cpu.write_ascii("take ");
                cpu.write_ascii(item);
                cpu.write_ascii("\n");
                cpu.expect_ascii(&format!(
                    "
You take the {}.

Command?
",
                    item
                ))?
            }
            cpu.write_ascii("east\n");
            match cpu.expect_ascii(
                "\n\n\n== Pressure-Sensitive Floor ==
Analyzing...

Doors here lead:
- west
\nA loud, robotic voice says \"Alert! Droids on this ship are ",
            ) {
                Ok(_) => {}
                Err(e) => {
                    dbg!(e);
                    dbg!(&items[0..len]);
                    cpu.read_ascii(true)?;
                    break 'main;
                }
            }

            cpu.read_ascii(false)?;
        }
        if !items.next_permutation() {
            break;
        }
    }
    // dbg!(results);
    // return Ok(0);

    loop {
        match cpu.read_ascii(true)? {
            CpuState::Halted => break,
            CpuState::NeedsInput => {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .map_err(|_| err_msg("Failed to read input"))?;
                input = match input.trim() {
                    "q" => break,
                    "n" => "north",
                    "s" => "south",
                    "e" => "east",
                    "w" => "west",
                    s => s,
                }
                .into();

                cpu.write_ascii(&input);
                cpu.write_ascii("\n");
            }
            CpuState::Output(_) => panic!("should have been printed"),
        }
    }
    Ok(0)
}

/*

    emag
egg mug  WD      SL
         Start
    jam| asteri  klein
cre phot es pod |lava
         -----     v
    cake tambou  lava
          v
          v      check  sensor
         loop    polyg


*/

fn part2(_input: &str) -> Result<i32> {
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
