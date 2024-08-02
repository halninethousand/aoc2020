fn main() {
    let input: Vec<&str> = include_str!("../../data/day12.txt")
        .lines()
        .collect();

    #[derive(Debug, Default)] 
    struct Ship {
        direction: i32,
        east_dist: u32,
        west_dist: u32,
        north_dist: u32,
        south_dist: u32
    }

    impl Ship {
        fn perform_instruction(&mut self, instruction: &str) {
            let dir = &instruction[..1];
            let amount: u32 = instruction[1..].parse().unwrap();
            // println!("dir: {}, amount: {}", dir, amount);
            
            if dir == "N" {
                self.north_dist += amount; 
            }
            if dir == "E" {
                self.east_dist += amount; 
            }
            if dir == "W" {
                self.west_dist += amount; 
            }
            if dir == "S" {
                self.south_dist += amount; 
            }
            if dir == "L" {
                self.direction = (self.direction + amount as i32) % 360;
            } else if dir == "R" {
                self.direction = ((self.direction - amount as i32)+ 360) % 360;
            }
            if dir == "F" {
                if self.direction == 90 {
                    self.north_dist += amount;
                }
                if self.direction == 0 {
                    self.east_dist += amount;
                }
                if self.direction == 180 {
                    self.west_dist += amount;
                }
                if self.direction == 270 {
                    self.south_dist += amount;
                }
            }
        }

        fn manhattan_distance(&self) -> i32 {
            (self.west_dist as i32 - self.east_dist as i32).abs() + (self.north_dist as i32 - self.south_dist as i32).abs()
        }
    }
    println!("{:?}", input);
    
    let mut ship = Ship::default();
    
    println!("{:?}", ship);
    
    for item in input {
        ship.perform_instruction(item);
    } 

    println!("Mahnattan distance: {} for ship: {:?}", ship.manhattan_distance(), ship);


}
