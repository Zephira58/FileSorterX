# Getting the core ubuntu OS
FROM rust
# Adding metadata
LABEL Maintainer=Xanthus58 Email=business@xanthus.uk Website=https://xanthus.uk/ Github=https://github.com/Xanthus58/FileSorterX
# Pulls my filesorterX binary applicaiton
RUN apt-get update -y
RUN apt install git -y
WORKDIR /filesorterx
RUN git clone https://github.com/Xanthus58/FileSorterX /filesorterx
VOLUME [ "/data" ]
ENV INPUT_DIR=/data
ENV OUTPUT_DIR=/data/Sorted
ENV COMMAND=sort
ADD .env /filesorterx/
RUN cargo build --release
RUN mv /filesorterx/target/release/FileSorterX /bin/filesorterx
CMD [ "filesorterx $INPUT_DIR $OUTPUT_DIR --verbose $COMMAND" ]
