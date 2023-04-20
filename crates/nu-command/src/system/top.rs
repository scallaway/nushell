use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    IntoPipelineData, PipelineData, ShellError, Signature, Value,
};
use std::time::Duration;

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
    // We just want the total process count here, so we don't need to wait long
    // at all
    let processes_count = nu_system::collect_proc(Duration::from_millis(1), false).len();

    // Order, by default, by CPU Usage
    // Sorted unstably for performance (since this is updated in real time)
    let _cpu_first_processes = processes
        .values()
        .collect::<Vec<&Process>>()
        .sort_unstable_by(|p1, p2| p1.cpu_usage().partial_cmp(&p2.cpu_usage()).unwrap());

    let columns = processes
        .values()
        .map(|process| process.name().to_string())
        .collect::<Vec<String>>();

    let values = processes
        .values()
        .map(|process| Value::String {
            // val: process.tasks.values().map(|task| task.name()).join(","),
            val: process.cpu_usage().to_string(),
            span,
        })
        .collect();

    let pipeline_data = Value::Record {
        cols: columns,
        vals: values,
        span: call.head,
    };

    // Will need to stream this data to the terminal if possible
    Ok(pipeline_data.into_pipeline_data())
}
