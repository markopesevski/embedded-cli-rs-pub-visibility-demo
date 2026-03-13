use cli::BaseCommand;

fn main() {
    let mut cli = embedded_cli::cli::CliBuilder::default()
        .build()
        .expect("Failed to build CLI");
    cli.process_byte::<BaseCommand, _>(
        0x00,
        &mut BaseCommand::processor(|_h, _c| match _c {
            BaseCommand::Command => Ok(()),
        }),
    )
    .unwrap();
}
