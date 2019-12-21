use aoc2019::coor::Coor;
use aoc2019::cpu::{set_memory, Cpu, CpuState};
use aoc2019::{dispatch, Result};
use failure::bail;
use std::collections::HashMap;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Scaffold,
    Space,
    RobotUp,
    RobotDown,
    RobotLeft,
    RobotRight,
}

fn read_map(cpu: &mut Cpu) -> Result<HashMap<Coor, Tile>> {
    let mut map = HashMap::new();
    let mut pos = Coor::new(0, 0);
    let mut last_was_newline = false;
    loop {
        match cpu.run()? {
            CpuState::Output(value) => {
                if value == '\n' as i64 {
                    if last_was_newline {
                        break Ok(map);
                    }
                    last_was_newline = true;
                    pos = Coor::new(0, pos.y + 1);
                } else {
                    last_was_newline = false;
                    let tile = match value as u8 as char {
                        '#' => Tile::Scaffold,
                        '.' => Tile::Space,
                        '^' => Tile::RobotUp,
                        '>' => Tile::RobotRight,
                        '<' => Tile::RobotLeft,
                        'v' => Tile::RobotDown,
                        c => bail!("invalid char: {}", c),
                        // _ => break Ok(map),
                    };
                    map.insert(pos, tile);
                    pos += Coor::new(1, 0);
                }
            }
            _ => break Ok(map),
        }
    }
}

fn part1(input: &str) -> Result<i64> {
    let mut cpu = Cpu::from_str(input);
    let map = read_map(&mut cpu)?;
    let mut alignment = 0;
    for coor in map.keys() {
        if *map.get(coor).unwrap_or(&Tile::Space) == Tile::Scaffold
            && *map.get(&(*coor + Coor::new(-1, 0))).unwrap_or(&Tile::Space) == Tile::Scaffold
            && *map.get(&(*coor + Coor::new(1, 0))).unwrap_or(&Tile::Space) == Tile::Scaffold
            && *map.get(&(*coor + Coor::new(0, 1))).unwrap_or(&Tile::Space) == Tile::Scaffold
            && *map.get(&(*coor + Coor::new(0, -1))).unwrap_or(&Tile::Space) == Tile::Scaffold
        {
            // dbg!(coor);
            alignment += coor.x * coor.y;
        }
    }
    // loop {
    // match cpu.run()? {
    // CpuState::Output(value) => print!("{}", value as u8 as char),
    // _ => break,
    // }
    // }
    Ok(alignment)
}

fn part2(input: &str) -> Result<i64> {
    let mut cpu = Cpu::from_str(input);
    set_memory(&mut cpu, 0, 2);
    //     .chars()
    //     .map(|c| c as u8 as i64)
    //     {
    //         cpu.enqueue_input(c);
    //     }
    let _map = read_map(&mut cpu)?;
    // loop {
    //     match cpu.run()? {
    //         CpuState::Output(value) => print!("{}", value as u8 as char),
    //         _ => break,
    //     }
    // }

    // L,10,R,8,R,8,
    // L,10,R,8,R,8,
    // L,10,L,12,R,8,R,10,R,10,L,12,R,10
    // L,10,L,12,R,8,R,10,R,10,L,12,R,10,
    // L,10,L,12,R,8,R,10,R,12,L,12,R,10,

    // R,10,L,12,R,10,
    // L,10,R,8,R,8

    cpu.expect_ascii("Main:\n")?;
    cpu.write_ascii("A,A,B,C,B,C,B,C,C,A\n");
    cpu.expect_ascii("Function A:\n")?;
    cpu.write_ascii("L,10,R,8,R,8\n");
    cpu.expect_ascii("Function B:\n")?;
    cpu.write_ascii("L,10,L,12,R,8,R,10\n");
    cpu.expect_ascii("Function C:\n")?;
    cpu.write_ascii("R,10,L,12,R,10\n");
    cpu.expect_ascii("Continuous video feed?\n")?;
    cpu.write_ascii("n\n");

    // while let CpuState::Output(value) = cpu.run()? {
    //     print!("{}", value as u8 as char);
    // }
    read_map(&mut cpu)?;

    while let CpuState::Output(value) = cpu.run()? {
        // print!("{}", value);
        return Ok(value);
    }
    // if let Ok(map) = read_map(&mut cpu) {}
    // dbg!(cpu.run()?);
    // dbg!(cpu.run()?);
    // dbg!(cpu.run()?);
    // let mut alignment = 0;

    // }
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
