use crate::pwg::input_to_value;
use acir::{
    AcirField, BlackBoxFunc,
    circuit::opcodes::FunctionInput,
    native_types::{Witness, WitnessMap},
};
use acvm_blackbox_solver::BigIntSolver;

use crate::pwg::OpcodeResolutionError;

/// Resolve BigInt opcodes by storing BigInt values (and their moduli) by their ID in the BigIntSolver
/// - When it encounters a bigint operation opcode, it performs the operation on the stored values
///   and store the result using the provided ID.
/// - When it gets a to_bytes opcode, it simply looks up the value and resolves the output witness accordingly.
pub(crate) struct AcvmBigIntSolver {
    bigint_solver: BigIntSolver,
}

impl AcvmBigIntSolver {
    pub(crate) fn with_pedantic_solving(pedantic_solving: bool) -> AcvmBigIntSolver {
        let bigint_solver = BigIntSolver::with_pedantic_solving(pedantic_solving);
        AcvmBigIntSolver { bigint_solver }
    }

    pub(crate) fn bigint_from_bytes<F: AcirField>(
        &mut self,
        inputs: &[FunctionInput<F>],
        modulus: &[u8],
        output: u32,
        initial_witness: &mut WitnessMap<F>,
    ) -> Result<(), OpcodeResolutionError<F>> {
        let bytes = inputs
            .iter()
            .map(|input| input_to_value(initial_witness, *input, false).unwrap().to_u128() as u8)
            .collect::<Vec<u8>>();
        self.bigint_solver.bigint_from_bytes(&bytes, modulus, output)?;
        Ok(())
    }

    pub(crate) fn bigint_to_bytes<F: AcirField>(
        &self,
        input: u32,
        outputs: &[Witness],
        initial_witness: &mut WitnessMap<F>,
    ) -> Result<(), OpcodeResolutionError<F>> {
        if self.bigint_solver.pedantic_solving() && outputs.len() != 32 {
            panic!("--pedantic-solving: bigint_to_bytes: outputs.len() != 32: {}", outputs.len());
        }
        let mut bytes = self.bigint_solver.bigint_to_bytes(input)?;
        while bytes.len() < outputs.len() {
            bytes.push(0);
        }
        bytes.iter().zip(outputs.iter()).for_each(|(byte, output)| {
            initial_witness.insert(*output, F::from(*byte as u128));
        });
        Ok(())
    }

    pub(crate) fn bigint_op<F>(
        &mut self,
        lhs: u32,
        rhs: u32,
        output: u32,
        func: BlackBoxFunc,
    ) -> Result<(), OpcodeResolutionError<F>> {
        self.bigint_solver.bigint_op(lhs, rhs, output, func)?;
        Ok(())
    }
}
