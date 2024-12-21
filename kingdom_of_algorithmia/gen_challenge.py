from pathlib import Path
import sys


class print_colors:
    OK = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'

def make_challenge_file(challenge_num: int):
    challenge_file = open(f"./src/challenge/c{challenge_num}.rs", "x")
    challenge_file.write(f"""
#[derive(Default)]
pub struct C{challenge_num} {{
    
}}

impl C{challenge_num} {{
    pub fn new() -> Self {{
        Self::default()
    }}
}}

impl Challenge for C{challenge_num} {{
    fn name(&self) -> String {{
        format!("")
    }}

    fn parse_input(&mut self) {{
        
    }}

    fn part1(&self) -> String {{
        format!("")
    }}

    fn part2(&self) -> String {{
        format!("")
    }}

    fn part3(&self) -> String {{
        format!("")
    }}
}}
    """)
    challenge_file.close()

def make_input_file(challenge_num: int, part_num: int):
    input_file = open(f"./src/input/c{challenge_num}p{part_num}.txt", "w")
    input_file.close()


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <challenge_num>")
    challenge_num = int(sys.argv[1])

    if not Path(f"./src/challenge/c{challenge_num}.rs").is_file():
        make_challenge_file(challenge_num)
        print(f"{print_colors.OK}generated ./src/challenge/c{challenge_num}.rs{print_colors.ENDC}")
    else:
        print(f"{print_colors.FAIL}file ./src/challenge/c{challenge_num}.rs already exists{print_colors.ENDC}")
        
    for part in range(3):

        if not Path(f"./src/input/c{challenge_num}p{part + 1}.txt").is_file():   
            make_input_file(challenge_num, part + 1)
            print(f"{print_colors.OK}generated ./input/c{challenge_num}p{part + 1}.txt{print_colors.ENDC}")
        else:
            print(f"{print_colors.FAIL}file ./input/c{challenge_num}p{part + 1}.txt already exists{print_colors.ENDC}")

