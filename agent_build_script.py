import os


def build_rust_binary(home, id):
    # Read the template
    with open('./agent/src/agent_config.rs.tmpl', 'r') as file:
        template = file.read()

    # Replace placeholders
    source_code = template.replace("{{ID}}", id)
    source_code = source_code.replace("{{HOME}}", home)

    # Write to a source file
    with open('./agent/src/agent_config.rs', 'w') as file:
        file.write(source_code)

    # Compile the binary
    os.system('cargo build --release --manifest-path=./agent/Cargo.toml')


# Example usage
build_rust_binary("http://127.0.0.1:8080", "8080")
