FROM alpine:latest

# Install necessary dependencies, then install Rust toolchain.
RUN apk add --no-cache curl gcc musl-dev
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s --  -y

# Add Rust to PATH, then update. If the update fails, we know that rust didn't install.
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup update

# The rest of this is from "Zero to Prod in Rust", Palmieri.
# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not 
# exist already.
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN pwd

# This copies the entire backend_kronos directory, which might be undesirable behvaior down the line 
# For now, this is fine. 
COPY . . 

RUN cargo build --release
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./target/release/backend_kronos"]