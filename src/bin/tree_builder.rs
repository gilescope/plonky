use plonky::{CurveAddGate, RescueStepBGate, RescueStepAGate, ArithmeticGate, ConstantGate, BufferGate, PublicInputGate, Base4SumGate, CurveEndoGate, CurveDblGate, Tweedledum, Tweedledee, Gate };
use plonky::custom_gates::tree_builder::gates_to_tree;

#[macro_use]
extern crate log;


fn main() {
    pretty_env_logger::init();
    type C = Tweedledum;
    type InnerC = Tweedledee;

    let gates = [
        (CurveAddGate::<C, InnerC>::NAME, CurveAddGate::<C, InnerC>::NUM_CONSTANTS, CurveAddGate::<C, InnerC>::DEGREE),
        (CurveDblGate::<C, InnerC>::NAME, CurveDblGate::<C, InnerC>::NUM_CONSTANTS, CurveDblGate::<C, InnerC>::DEGREE),
        (CurveEndoGate::<C, InnerC>::NAME, CurveEndoGate::<C, InnerC>::NUM_CONSTANTS, CurveEndoGate::<C, InnerC>::DEGREE),
        (Base4SumGate::<C>::NAME, Base4SumGate::<C>::NUM_CONSTANTS, Base4SumGate::<C>::DEGREE),
        (PublicInputGate::<C>::NAME, PublicInputGate::<C>::NUM_CONSTANTS, PublicInputGate::<C>::DEGREE),
        (BufferGate::<C>::NAME, BufferGate::<C>::NUM_CONSTANTS, BufferGate::<C>::DEGREE),
        (ConstantGate::<C>::NAME, ConstantGate::<C>::NUM_CONSTANTS, ConstantGate::<C>::DEGREE),
        (ArithmeticGate::<C>::NAME, ArithmeticGate::<C>::NUM_CONSTANTS, ArithmeticGate::<C>::DEGREE),
        (RescueStepAGate::<C>::NAME, RescueStepAGate::<C>::NUM_CONSTANTS, RescueStepAGate::<C>::DEGREE),
        (RescueStepBGate::<C>::NAME, RescueStepBGate::<C>::NUM_CONSTANTS, RescueStepBGate::<C>::DEGREE),
    ];

    let tree = gates_to_tree(&gates);
    // tree.graph();
    dbg!(tree.prefixes());
}