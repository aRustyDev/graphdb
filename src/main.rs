mod cli;
mod data_structs;
mod env;
// mod storage::{distributed, mem};
// mod api::api;
// mod auth::{jwt, oidc};
// mod algos::algorithms;

use clap::Parser;
// use log::{error, Level};
// use opentelemetry::KeyValue;
// use opentelemetry_appender_log::OpenTelemetryLogBridge;
// use opentelemetry_sdk::logs::LoggerProvider;
// use opentelemetry_sdk::Resource;
// use opentelemetry_semantic_conventions::resource::SERVICE_NAME;

fn main() {
    let args = cli::Args::parse();

    println!("Hello {}!", args.local);

    // // Setup LoggerProvider with a stdout exporter
    // let exporter = opentelemetry_stdout::LogExporter::default();
    // let logger_provider = LoggerProvider::builder()
    //     .with_resource(Resource::new([KeyValue::new(
    //         SERVICE_NAME,
    //         "logs-basic-example",
    //     )]))
    //     .with_simple_exporter(exporter)
    //     .build();

    // // Setup Log Appender for the log crate.
    // let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    // log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();
    // log::set_max_level(Level::Error.to_level_filter());

    // // Emit logs using macros from the log crate.
    // // These logs gets piped through OpenTelemetry bridge and gets exported to stdout.
    // error!(target: "my-target", "hello from {}. My price is {}", "apple", 2.99);
}