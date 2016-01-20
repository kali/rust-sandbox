
extern crate time;

use MachineMove::*;

#[derive(Copy,Clone)]
enum MachineMove {
    Left = -1,
    Stay = 0,
    Right = 1
}

#[derive(Copy,Clone)]
struct MachineTransition {
    write:bool,
    mov:MachineMove,
    switch:isize
}

#[derive(Copy,Clone)]
struct MachineState {
    zero:MachineTransition,
    one:MachineTransition
}

#[derive(Copy,Clone)]
struct MachineDefinition<'a> {
    states:&'a[MachineState]
}

struct Machine<'a> {
    definition: &'a MachineDefinition<'a>,
    state: isize,
    position: isize,
    tape_left:Vec<bool>,
    tape_right:Vec<bool>
}

impl<'a> Machine<'a> {
/*
    #[allow(dead_code)]
    fn dump(&self) -> String {
        let dumped_right_tape:String = self.tape_right.iter().map(|&b| if b { '1' } else { '0' } ).collect();
        let dumped_left_tape:String = self.tape_left.iter().map(|&b| if b { '1' } else { '0' } ).rev().collect();
        let index:usize = (dumped_left_tape.len() as int + self.position + 1) as usize;
        let tape_as_string = String::from_str(" ")+dumped_left_tape+dumped_right_tape+" ";
        let tape = tape_as_string.as_slice();
        format!("s:{} tape:{:s}[{:s}]{:s}", self.state,
            tape.slice(0,index), tape.slice(index, index+1), tape.slice(index+1, tape.len()))
    }
*/
    fn step(&mut self) {
        if self.state < 0 {
            return
        }
        let cell:&mut bool = if self.position >= 0 {
            let index:usize = self.position as usize;
            if index == self.tape_right.len() {
                self.tape_right.push(false);
            }
            self.tape_right.get_mut(index).unwrap()
        } else {
            let index:usize = (-self.position-1) as usize;
            if index == self.tape_left.len() {
                self.tape_left.push(false);
            }
            self.tape_left.get_mut(index).unwrap()
        };
        let state = self.definition.states[self.state as usize];
        let transition = if *cell { state.one } else { state.zero };
        *cell = transition.write;
        self.state = transition.switch;
        self.position += transition.mov as isize
    }
}

fn main() {
    let states:&[MachineState;5] = &[
/* 2 states
            MachineState {
                zero:MachineTransition { write:true, mov:Right, switch:1 },
                one:MachineTransition { write:true, mov:Left, switch:1 }},
            MachineState {
                zero:MachineTransition { write:true, mov:Left, switch:0 },
                one:MachineTransition { write:true, mov:Right, switch:-1 }}
*/
/* 3 states
            MachineState {
                zero:MachineTransition { write:true, mov:Right, switch:1 },
                one:MachineTransition { write:true, mov:Right, switch:-1 }},
            MachineState {
                zero:MachineTransition { write:false, mov:Right, switch:2 },
                one:MachineTransition { write:true, mov:Right, switch:1 }},
            MachineState {
                zero:MachineTransition { write:true, mov:Left, switch:2 },
                one:MachineTransition { write:true, mov:Left, switch:0 }}
*/
/* 4 states
            MachineState {
                zero:MachineTransition { write:true, mov:Right, switch:1 },
                one:MachineTransition { write:true, mov:Left, switch:1 }},
            MachineState {
                zero:MachineTransition { write:true, mov:Left, switch:0 },
                one:MachineTransition { write:false, mov:Left, switch:2 }},
            MachineState {
                zero:MachineTransition { write:true, mov:Right, switch:-1 },
                one:MachineTransition { write:true, mov:Left, switch:3 }},
            MachineState {
                zero:MachineTransition { write:true, mov:Right, switch:3 },
                one:MachineTransition { write:false, mov:Right, switch:0 }}
*/
/* 5 states */
            MachineState {
                zero:MachineTransition { write:true, mov:Right, switch:1 },
                one:MachineTransition { write:true, mov:Left, switch:2 }},
            MachineState {
                zero:MachineTransition { write:true, mov:Right, switch:2 },
                one:MachineTransition { write:true, mov:Right, switch:1 }},
            MachineState {
                zero:MachineTransition { write:true, mov:Right, switch:3 },
                one:MachineTransition { write:false, mov:Left, switch:4 }},
            MachineState {
                zero:MachineTransition { write:true, mov:Left, switch:0 },
                one:MachineTransition { write:true, mov:Left, switch:3 }},
            MachineState {
                zero:MachineTransition { write:true, mov:Right, switch:-1 },
                one:MachineTransition { write:false, mov:Left, switch:0 }}
        ];
    let def = MachineDefinition { states: states };
    for it in (1usize..10) {
        let start = time::precise_time_s();
        let mut machine:Machine = Machine { definition: &def, state:0, position: 0, tape_left:vec![], tape_right:vec![false] };
        let mut i:usize = 0;
        while machine.state != -1 {
    //        prisizeln!("{:9u} {}", i, machine.dump())
            machine.step();
            i+=1;
        }
        let end = time::precise_time_s();
        println!("{:9} steps in {}s", i, end-start);
    }
//    prisizeln!("{:9u} {}", i, machine.dump())
}
