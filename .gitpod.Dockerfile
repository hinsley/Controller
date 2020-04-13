FROM gitpod/workspace-full-vnc
                    
USER gitpod

RUN sudo apt-get update -q \
    && sudo apt-get install libgtk-3-dev

# TODO: Step 2 from https://github.com/sciter-sdk/rust-sciter#getting-started
RUN wget https://sciter.com/sdk/sciter-sdk.zip \
    && mkdir sciter-sdk \
    && unzip sciter-sdk.zip -d sciter-sdk \
    && rm sciter-sdk.zip
