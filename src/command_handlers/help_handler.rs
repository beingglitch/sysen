pub fn handle_help_command() {
    println!("\nSysen — Understand your system\n");

    println!("Usage:");
    println!("  sysen <command>\n");

    println!("Commands:");
    println!("  status     Show system status (CPU, memory, processes)");
    println!("  explain    Explain system behavior and issues");
    println!("  help       Show this help message\n");

    println!("Examples:");
    println!("  sysen status");
    println!("  sysen explain\n");
}