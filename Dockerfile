FROM rust:1.88-slim AS rust-build

WORKDIR /build

COPY . .

RUN apt-get update && apt-get install -y \
    clang \
    libclang-dev \
    llvm-dev \
    pkg-config \
    make \
    perl \
    libperl-dev \
    libaspell-dev \
    libhunspell-dev \
    zlib1g-dev \
    curl \
    git \
    build-essential \
    cpanminus && \
    rm -rf /var/lib/apt/lists/*

# Install Hunspell and Irish dictionary from Debian
RUN apt-get update && apt-get install -y --no-install-recommends \
    hunspell \
    libhunspell-dev && \
    rm -rf /var/lib/apt/lists/*

# Build ga_IE.aff and ga_IE.dic from GaelSpell repo without invoking its Makefile
RUN git clone --depth=1 https://github.com/kscanne/gaelspell /tmp/gaelspell && \
    cd /tmp/gaelspell && \
    make giorr && \
    cat myspell-header hunspell-header > myspelltemp.txt && \
    ./ispellaff2myspell --charset=latin1 gaeilge.aff --myheader myspelltemp.txt \
      | sed 's/""/0/' \
      | sed '40,$s/"//g' \
      | perl -p -e 's/^PFX S( +)([a-z])( +)[a-z]h( +)[a-z](.*)/print "PFX S$1$2$3$2h$4$2$5\nPFX S$1\u$2$3\u$2h$4\u$2$5";/e' \
      | sed 's/S Y 9$/S Y 18/' \
      | sed 's/\([]A-Z]\)1$/\1/' > ga_IE.aff && \
    bash -lc 'LC_ALL=C sort -u gaeilge.raw aitiuil ceol eachtar gall gallainm-b gallainm-f giorr gno lit logainm miotas stair treise ainm-b ainm-f bioblabeag daoine latecaps > ga_IE.dic' && \
    sh -lc 'printf "%s\n" $(wc -l < ga_IE.dic) | cat - ga_IE.dic > ga_IE.dic.tmp && mv ga_IE.dic.tmp ga_IE.dic' && \
    install -d /usr/share/hunspell && \
    install -m 0644 ga_IE.dic /usr/share/hunspell/ga_IE.dic && \
    install -m 0644 ga_IE.aff /usr/share/hunspell/ga_IE.aff

ENV PERL_CARTON_PATH=/opt/perl5 PERL5LIB=/opt/perl5/lib/perl5

# Install GaelSpellBridge Perl module from deps (Text::Hunspell)
RUN cpanm --notest --local-lib=/opt/perl5 Text::Hunspell && \
    cpanm --notest --local-lib=/opt/perl5 ./deps/GaelSpellBridge

RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends perl libstdc++6 zlib1g curl libhunspell-1.7-0 \
 && rm -rf /var/lib/apt/lists/*
ENV PERL5LIB=/opt/perl5/lib/perl5
COPY --from=rust-build /opt/perl5 /opt/perl5
COPY --from=rust-build /build/target/release/gaelspell-server /usr/local/bin/gaelspell-server
COPY --from=rust-build /usr/share/hunspell/ga_IE.* /usr/share/hunspell/
EXPOSE 5000
CMD ["gaelspell-server"]


