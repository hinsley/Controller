FROM gitpod/workspace-full-vnc
                    
USER gitpod

RUN sudo apt-get update \
    && sudo apt-get install libgtk-3-dev

# TODO: Step 2 from https://github.com/sciter-sdk/rust-sciter#getting-started
RUN wget https://sciter.com/sdk/sciter-sdk.zip \
    && mkdir sciter-sdk \
    && unzip sciter-sdk.zip -d sciter-sdk \
    && rm sciter-sdk.zip

# Install custom tools, runtime, etc. using apt-get
# For example, the command below would install "bastet" - a command line tetris clone:
#
# RUN sudo apt-get -q update && #     sudo apt-get install -yq bastet && #     sudo rm -rf /var/lib/apt/lists/*
#
# More information: https://www.gitpod.io/docs/config-docker/
