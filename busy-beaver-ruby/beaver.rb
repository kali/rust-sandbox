
LEFT = -1
STAY = 0
RIGHT = 1

class MachineTransition
  attr_accessor :write, :move, :switch
  def initialize(w,m,s)
    @write = w
    @move = m
    @switch = s
  end

end

class MachineState
  attr_accessor :zero, :one
  def initialize(zero,one)
    @zero = zero
    @one = one
  end
end

class Machine
  attr_accessor :state, :definition
  def initialize(definition)
    @definition = definition
    @state = 0
    @position = 0
    @tape_left = []
    @tape_right = []
  end

  def step
    return if @state < 0
    if @position >= 0
      (tape, position) = [@tape_right, @position]
    else
      (tape, position) = [@tape_left, -@position-1]
    end
    tape[position] == false if position > tape.size
    s = @definition[@state]
    t = if tape[position] then s.one else s.zero end

    tape[position] = t.write
    @state = t.switch
    @position += t.move
  end
end

states = [
#2 states
#         MachineState::new(
#             MachineTransition::new(true,RIGHT,1),
#             MachineTransition::new(true,LEFT,1)),
#         MachineState::new(
#             MachineTransition::new(true,LEFT,0),
#             MachineTransition::new(true,RIGHT,-1))
#3 states
#         MachineState {
#             zero:MachineTransition { write:true, mov:Right, switch:1 },
#             one:MachineTransition { write:true, mov:Right, switch:-1 }},
#         MachineState {
#             zero:MachineTransition { write:false, mov:Right, switch:2 },
#             one:MachineTransition { write:true, mov:Right, switch:1 }},
#         MachineState {
#             zero:MachineTransition { write:true, mov:Left, switch:2 },
#             one:MachineTransition { write:true, mov:Left, switch:0 }}
#
#4 states
#         MachineState {
#             zero:MachineTransition { write:true, mov:Right, switch:1 },
#             one:MachineTransition { write:true, mov:Left, switch:1 }},
#         MachineState {
#             zero:MachineTransition { write:true, mov:Left, switch:0 },
#             one:MachineTransition { write:false, mov:Left, switch:2 }},
#         MachineState {
#             zero:MachineTransition { write:true, mov:Right, switch:-1 },
#             one:MachineTransition { write:true, mov:Left, switch:3 }},
#         MachineState {
#             zero:MachineTransition { write:true, mov:Right, switch:3 },
#             one:MachineTransition { write:false, mov:Right, switch:0 }}
#
#5 states */
#
            MachineState::new(
                MachineTransition::new( true, RIGHT, 1),
                MachineTransition::new( true, LEFT, 2 )),
            MachineState::new(
                MachineTransition::new( true, RIGHT, 2),
                MachineTransition::new( true, RIGHT, 1)),
            MachineState::new(
                MachineTransition::new( true, RIGHT, 3),
                MachineTransition::new( false, LEFT, 4)),
            MachineState::new(
                MachineTransition::new( true, LEFT, 0),
                MachineTransition::new( true, LEFT, 3)),
            MachineState::new(
                MachineTransition::new( true, RIGHT, -1),
                MachineTransition::new( false, LEFT, 0))
]
    m = Machine::new(states)
    i = 0
    while m.state != -1 do
      m.step
      i+=1
    end
    puts i
