use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    PipelineData, ShellError, Signature,
};

#[derive(Clone)]
pub struct Top;

impl Command for Top {
    fn name(&self) -> &str {
        "top"
    }

    fn signature(&self) -> Signature {
        Signature::build("top")
    }

    fn usage(&self) -> &str {
        "View live information about a running system."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        run_top(engine_state)
    }
}

fn run_top(engine_state: &EngineState) -> Result<PipelineData, ShellError> {
    Ok("Called from top"
        .bytes()
        .into_iter()
        .into_pipeline_data(engine_state.ctrlc.clone()))
}
