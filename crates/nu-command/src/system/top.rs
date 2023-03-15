use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, Value,
};
use sysinfo::SystemExt;

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
        _engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        run_top(call)
    }
}

fn run_top(call: &Call) -> Result<PipelineData, ShellError> {
    // Eventually this will have to be refreshed
    let system = sysinfo::System::new_all();
    let processes_count = system.processes().values().len();

    let pipeline_data = Value::String {
        val: format!("Total processes: {:?}", processes_count).to_string(),
        span: call.span(),
    };

    // Will need to stream this data to the terminal if possible
    Ok(pipeline_data.into_pipeline_data())
}
