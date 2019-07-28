package instruction

type RegisterBank map[string]int

func NewRegisterBank() RegisterBank {
	return make(map[string]int)
}

func (rb RegisterBank) Evaluate(ir JumpInstruction) {
	ir.evaluateOn(rb)
}

func (rb RegisterBank) EvaluateAll(irs []JumpInstruction) {
	for _, ir := range irs {
		ir.evaluateOn(rb)
	}
}
