#Use microsoft devcontainer
ARG VARIANT=ubuntu-22.04
FROM mcr.microsoft.com/vscode/devcontainers/base:0-${VARIANT}

# Update the package manager and install required dependencies
RUN apt-get update && apt-get install -y curl

# Install Rust using Rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Rust binaries to the PATH
ENV PATH=/root/.cargo/bin:$PATH

# Verify the Rust installation
RUN rustc --version && cargo --version

# Set the working directory inside the container
WORKDIR /workspace

# Define the entry point
CMD ["/bin/bash", "-c", "if [ \"$EXECUTE_BIN_BASH\" -eq 1 ]; then /bin/bash; else echo \"Not executing /bin/bash\"; fi"]