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

private class Machine {
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
        return "state:\(state) position:\(position) tape: \(tapeLeft) \(tapeRight)"
    }

    func step() {
        if(state == -1) {
            return
        }
        if(position >= 0) {
            if(position >= tapeRight.count) {
                tapeRight.append(false)
            }
            let read = tapeRight[position]
            let s = definition.states[state]
            let transition = read ? s.one : s.zero
            tapeRight[position] = transition.write
            state = transition.switchTo
            position = position + transition.move.rawValue
        } else {
            if(-position-1 >= tapeLeft.count) {
                tapeLeft.append(false)
            }
            let read = tapeLeft[-position-1]
            let s = definition.states[state]
            let transition = read ? s.one : s.zero
            tapeLeft[-position-1] = transition.write
            state = transition.switchTo
            position = position + transition.move.rawValue
        }
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
for index in 0...10 {
    var i = 0
    let machine = Machine(def:definition)
    while(machine.state != -1) {
        machine.step()
        i+=1
    }
}
