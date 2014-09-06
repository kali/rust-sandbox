package main

import "fmt"
import "time"

type MachineMove int

const (
    Left MachineMove = -1
    Stay MachineMove = 0
    Right MachineMove = 1
)

type MachineTransition struct {
    write    bool
    move     MachineMove
    switchTo int
}

type MachineState struct {
    zero MachineTransition
    one  MachineTransition
}

type MachineDefinition struct {
    states []MachineState
}

type Machine struct {
    definition MachineDefinition
    state int
    position int
    tapeLeft []bool
    tapeRight []bool
}

func (self *Machine) dump() string {
    return fmt.Sprintf("state:%d position:%d", self.state, self.position);
}
/*
        let dumped_right_tape:String = self.tape_right.iter().map(|&b| if b { '1' } else { '0' } ).collect();
        let dumped_left_tape:String = self.tape_left.iter().map(|&b| if b { '1' } else { '0' } ).rev().collect();
        let index:uint = (dumped_left_tape.len() as int + self.position + 1) as uint;
        let tape_as_string = String::from_str(" ")+dumped_left_tape+dumped_right_tape+" ";
        let tape = tape_as_string.as_slice();
        format!("s:{} tape:{:s}[{:s}]{:s}", self.state,
            tape.slice(0,index), tape.slice(index, index+1), tape.slice(index+1, tape.len()))
*/

func (self *Machine) step() {
    if(self.state < 0) {
        return;
    }
    var tape []bool
    var index int
    if(self.position >= 0) {
        index = self.position
        if(index == len(self.tapeRight)) {
            self.tapeRight = append(self.tapeRight, false)
        }
        tape = self.tapeRight
    } else {
        index = -self.position-1
        if(index == len(self.tapeLeft)) {
            self.tapeLeft = append(self.tapeLeft, false)
        }
        tape = self.tapeLeft
    }
    var read = tape[index]
    var state = self.definition.states[self.state]
    var transition MachineTransition
    if(read) {
        transition = state.one
    } else {
        transition = state.zero
    }
    tape[index] = transition.write
    self.state = transition.switchTo
    self.position += int(transition.move)
}

func main() {
    var states []MachineState = []MachineState {
        MachineState {
            zero:MachineTransition { write:true, move:Right, switchTo:1 },
            one:MachineTransition { write:true, move:Left, switchTo:2 }},
        MachineState {
            zero:MachineTransition { write:true, move:Right, switchTo:2 },
            one:MachineTransition { write:true, move:Right, switchTo:1 }},
        MachineState {
            zero:MachineTransition { write:true, move:Right, switchTo:3 },
            one:MachineTransition { write:false, move:Left, switchTo:4 }},
        MachineState {
            zero:MachineTransition { write:true, move:Left, switchTo:0 },
            one:MachineTransition { write:true, move:Left, switchTo:3 }},
        MachineState {
            zero:MachineTransition { write:true, move:Right, switchTo:-1 },
            one:MachineTransition { write:false, move:Left, switchTo:0 }},
    };
    var def = MachineDefinition { states: states }
    for j := 1;  j<=10; j++ {
        var before = time.Now()
        var machine = Machine { definition: def, position:0, state:0}
        var i = 0
        for ; machine.state != -1 ;  {
            machine.step()
            i+=1
        }
        var after = time.Now()
        fmt.Println(i, after.Sub(before))
    }
}
