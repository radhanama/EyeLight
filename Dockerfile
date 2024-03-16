FROM rust:latest

# Set locale
RUN apt-get update && apt-get install -y locales && \
    sed -i -e 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen && \
    locale-gen
ENV LANG en_US.UTF-8  
ENV LANGUAGE en_US:en  
ENV LC_ALL en_US.UTF-8   

# Install PostgreSQL
RUN apt-get update && \
    apt-get install -y postgresql postgresql-contrib

# Install additional dependencies for Rust development
RUN apt-get install -y openssh-server

# Configure SSH
RUN mkdir /var/run/sshd
RUN echo 'root:password' | chpasswd
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config

# Install Cargo
RUN apt-get install -y curl && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    export PATH="$HOME/.cargo/bin:$PATH" && \
    rustup default stable

# Expose PostgreSQL port
EXPOSE 5432

# Expose SSH port
EXPOSE 22

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the Rust application folder from the host to the container
COPY . .

# Start PostgreSQL and SSH services
CMD service postgresql start && /usr/sbin/sshd -D
