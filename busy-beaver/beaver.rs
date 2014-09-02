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
    tapeLeft:Vec<bool>,
    tapeRight:Vec<bool>
}

impl<'a> Machine<'a> {
    fn dump(&self) -> String {
        let dumpedRightTape:String = self.tapeRight.iter().map(|&b| if b { '1' } else { '0' } ).collect();
        let dumpedLeftTape:String = self.tapeLeft.iter().map(|&b| if b { '1' } else { '0' } ).rev().collect();
        let index:uint = (dumpedLeftTape.len() as int + self.position + 1) as uint;
        let tapeAsString = String::from_str(" ")+dumpedLeftTape+dumpedRightTape+" ";
        let tape = tapeAsString.as_slice();
        format!("s:{} tape:{:s}[{:s}]{:s}", self.state,
            tape.slice(0,index), tape.slice(index, index+1), tape.slice(index+1, tape.len()))
    }

    fn step(&mut self) {
        if self.state < 0 {
            return
        }
        let read:bool = if self.position >= 0 {
            let index:uint = self.position as uint;
            if index == self.tapeRight.len() {
                self.tapeRight.grow_set(index, &false, false);
            }
            self.tapeRight[index]
        } else {
            let index:uint = (-self.position-1) as uint;
            if index == self.tapeLeft.len() {
                self.tapeLeft.grow_set(index, &false, false);
            }
            self.tapeLeft[index]
        };
        let state = self.definition.states[self.state as uint];
        let transition = if read { state.one } else { state.zero };
        if self.position >= 0 {
            let index:uint = self.position as uint;
            self.tapeRight.grow_set(index, &false, transition.write);
        } else {
            let index:uint = (-self.position-1) as uint;
            self.tapeLeft.grow_set(index, &false, transition.write);
        };
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
    for it in range(1u,100) {
        let start = time::precise_time_s();
        let mut machine:Machine = Machine { definition: &def, state:0, position: 0, tapeLeft:vec![], tapeRight:vec![false] };
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
