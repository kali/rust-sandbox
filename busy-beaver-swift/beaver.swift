enum MachineMove:Int {
    case Left = -1, Stay = 0, Right = 1
}

struct MachineTransition {
    let write:Bool
    let move:MachineMove
    let switchTo:Int
}

struct MachineState {
    let zero:MachineTransition
    let one:MachineTransition
}

class MachineDefinition {
    let states:[MachineState]
    init(s:[MachineState]) {
        states = s
    }
}

class Machine {
    let definition: MachineDefinition
    var state: Int = 0
    var position: Int = 0
    var tapeLeft:[Bool] = []
    var tapeRight:[Bool] = []

    init(def:MachineDefinition) {
        definition = def
    }

    func toString() -> String {
/*
        s"state:$state position:$position tape:" +
        tapeLeft.reverseMap( if(_) '1' else '0' ).mkString +
        '|' + tapeRight.map( if(_) '1' else '0').mkString
*/
        "state:\(state) position:\(position) tape:"
    }

    func step() {
        if(state == -1) {
            return
        }
        var (tape,index) = position>=0 ? (tapeRight,position) : (tapeLeft, -position-1)
        if(index == tape.count) {
            tape.append(false)
        }
        let read = tape[index]
        let s = definition.states[state]
        let transition = read ? s.one : s.zero
        tape[index] = transition.write
        state = transition.switchTo
        position = position + transition.move
    }
}

let definition = MachineDefinition(s:[
    MachineState (
        zero:MachineTransition (write:true,move:MachineMove.Right,switchTo:1),
        one :MachineTransition (write:true,move:MachineMove.Left,switchTo:2)),
    MachineState (
        zero:MachineTransition (write:true,move:MachineMove.Right,switchTo:2),
        one :MachineTransition (write:true,move:MachineMove.Right,switchTo:1)),
    MachineState (
        zero:MachineTransition (write:true,move:MachineMove.Right,switchTo:3),
        one :MachineTransition (write:false,move:MachineMove.Left,switchTo:4)),
    MachineState (
        zero:MachineTransition (write:true,move:MachineMove.Left,switchTo:0),
        one :MachineTransition (write:true,move:MachineMove.Left,switchTo:3)),
    MachineState (
        zero:MachineTransition (write:true,move:MachineMove.Right,switchTo:-1),
        one :MachineTransition (write:false,move:MachineMove.Left,switchTo:0))
    ])
for index in 0...100 {
    var i = 0
    let machine = Machine(def:definition)
    while(machine.state != -1) {
        machine.step()
        i+=1
    }
    println(i)
}
