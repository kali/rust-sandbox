
extern crate debug;
extern crate time;

use std::string::String;

enum MachineMove {
    Left = -1,
    Stay = 0,
    Right = 1
}

struct MachineTransition {
    write:bool,
    move:MachineMove,
    switch:int
}

struct MachineState {
    zero:MachineTransition,
    one:MachineTransition
}

struct MachineDefinition {
    states:Vec<MachineState>
}

struct Machine<'a> {
    definition: &'a MachineDefinition,
    state: int,
    position: int,
    tape_left:Vec<bool>,
    tape_right:Vec<bool>
}

impl<'a> Machine<'a> {
    #[allow(dead_code)]
    fn dump(&self) -> String {
        let dumped_right_tape:String = self.tape_right.iter().map(|&b| if b { '1' } else { '0' } ).collect();
        let dumped_left_tape:String = self.tape_left.iter().map(|&b| if b { '1' } else { '0' } ).rev().collect();
        let index:uint = (dumped_left_tape.len() as int + self.position + 1) as uint;
        let tape_as_string = String::from_str(" ")+dumped_left_tape+dumped_right_tape+" ";
        let tape = tape_as_string.as_slice();
        format!("s:{} tape:{:s}[{:s}]{:s}", self.state,
            tape.slice(0,index), tape.slice(index, index+1), tape.slice(index+1, tape.len()))
    }

    fn step(&mut self) {
        if self.state < 0 {
            return
        }
        let cell:&mut bool = if self.position >= 0 {
            let index:uint = self.position as uint;
            if index == self.tape_right.len() {
                self.tape_right.grow_set(index, &false, false);
            }
            self.tape_right.get_mut(index)
        } else {
            let index:uint = (-self.position-1) as uint;
            if index == self.tape_left.len() {
                self.tape_left.grow_set(index, &false, false);
            }
            self.tape_left.get_mut(index)
        };
        let state = self.definition.states[self.state as uint];
        let transition = if *cell { state.one } else { state.zero };
        *cell = transition.write;
        self.state = transition.switch;
        self.position += transition.move as int
    }
}

fn main() {
    let def = MachineDefinition {
        states: vec![
/* 2 states
            MachineState {
                zero:MachineTransition { write:true, move:Right, switch:1 },
                one:MachineTransition { write:true, move:Left, switch:1 }},
            MachineState {
                zero:MachineTransition { write:true, move:Left, switch:0 },
                one:MachineTransition { write:true, move:Right, switch:-1 }}
*/
/* 3 states
            MachineState {
                zero:MachineTransition { write:true, move:Right, switch:1 },
                one:MachineTransition { write:true, move:Right, switch:-1 }},
            MachineState {
                zero:MachineTransition { write:false, move:Right, switch:2 },
                one:MachineTransition { write:true, move:Right, switch:1 }},
            MachineState {
                zero:MachineTransition { write:true, move:Left, switch:2 },
                one:MachineTransition { write:true, move:Left, switch:0 }}
*/
/* 4 states
            MachineState {
                zero:MachineTransition { write:true, move:Right, switch:1 },
                one:MachineTransition { write:true, move:Left, switch:1 }},
            MachineState {
                zero:MachineTransition { write:true, move:Left, switch:0 },
                one:MachineTransition { write:false, move:Left, switch:2 }},
            MachineState {
                zero:MachineTransition { write:true, move:Right, switch:-1 },
                one:MachineTransition { write:true, move:Left, switch:3 }},
            MachineState {
                zero:MachineTransition { write:true, move:Right, switch:3 },
                one:MachineTransition { write:false, move:Right, switch:0 }}
*/
/* 5 states */
            MachineState {
                zero:MachineTransition { write:true, move:Right, switch:1 },
                one:MachineTransition { write:true, move:Left, switch:2 }},
            MachineState {
                zero:MachineTransition { write:true, move:Right, switch:2 },
                one:MachineTransition { write:true, move:Right, switch:1 }},
            MachineState {
                zero:MachineTransition { write:true, move:Right, switch:3 },
                one:MachineTransition { write:false, move:Left, switch:4 }},
            MachineState {
                zero:MachineTransition { write:true, move:Left, switch:0 },
                one:MachineTransition { write:true, move:Left, switch:3 }},
            MachineState {
                zero:MachineTransition { write:true, move:Right, switch:-1 },
                one:MachineTransition { write:false, move:Left, switch:0 }}
        ]
    };
    for it in range(1u,10) {
        let start = time::precise_time_s();
        let mut machine:Machine = Machine { definition: &def, state:0, position: 0, tape_left:vec![], tape_right:vec![false] };
        let mut i:uint = 0;
        while machine.state != -1 {
    //        println!("{:9u} {}", i, machine.dump())
            machine.step();
            i+=1;
        }
        let end = time::precise_time_s();
        println!("{:9u} steps in {}s", i, end-start);
    }
//    println!("{:9u} {}", i, machine.dump())
}
