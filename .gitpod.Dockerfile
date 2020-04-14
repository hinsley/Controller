FROM gitpod/workspace-full-vnc
                    
USER gitpod

RUN sudo apt-get update \
    && sudo apt-get -y install libgtk-3-dev

RUN wget https://sciter.com/sdk/sciter-sdk.zip \
    && mkdir sciter-sdk \
    && unzip sciter-sdk.zip -d sciter-sdk \
    && rm sciter-sdk.zip

ENV PATH="/home/gitpod/sciter-sdk/bin.lnx:${PATH}"
