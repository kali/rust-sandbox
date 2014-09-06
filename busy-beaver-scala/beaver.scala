import scala.collection.mutable.Buffer

abstract sealed class MachineMove(val value:Int)
case object Left extends MachineMove(-1)
case object Stay extends MachineMove(0)
case object Right extends MachineMove(1)

case class MachineTransition(write:Boolean, move:MachineMove, switch:Int)
case class MachineState(zero:MachineTransition, one:MachineTransition)
case class MachineDefinition(states:IndexedSeq[MachineState])

class Machine(val definition:MachineDefinition) {
    var state = 0
    var position = 0
    val tapeLeft = Buffer(false)
    val tapeRight = Buffer(false)

    override def toString():String = {
        s"state:$state position:$position tape:" +
        tapeLeft.reverseMap( if(_) '1' else '0' ).mkString +
        '|' + tapeRight.map( if(_) '1' else '0').mkString
    }

    def step {
        if(state == -1)
            return
        val (tape,index) = if(position>=0)
            (tapeRight,position)
        else
            (tapeLeft, -position-1)
        if(index >= tape.size) {
            tape += false
        }
        val read = tape(index)
        val s = definition.states(state)
        val transition = if(read) s.one else s.zero
        tape(index) = transition.write
        state = transition.switch
        position += transition.move.value
    }
}



object Beaver {
    def main(args: Array[String]) {
        val definition = MachineDefinition(Array(
            MachineState (
                MachineTransition (true,Right,1),
                MachineTransition (true,Left,2)),
            MachineState (
                MachineTransition (true,Right,2),
                MachineTransition (true,Right,1)),
            MachineState (
                MachineTransition (true,Right,3),
                MachineTransition (false,Left,4)),
            MachineState (
                MachineTransition (true,Left,0),
                MachineTransition (true,Left,3)),
            MachineState (
                MachineTransition (true,Right,-1),
                MachineTransition (false,Left,0))
        ))
        (0 until 100).foreach { index =>
            var i = 0
            val start = System.currentTimeMillis
            val machine = new Machine(definition)
            while(machine.state != -1) {
                machine.step
                i+=1
            }
            println(s"i:$i " + (System.currentTimeMillis - start));
        }
    }
}
