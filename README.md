# Example: OpenTitan repo as a bazel dependency

## OpenTitanTool

Opentitantool can be built from this repo like this:

```
bazel build @lowrisc_opentitan//sw/host/opentitantool
```

## A stand-alone binary

A stand-alone bare-metal "Hello World" program can be built and signed for the
FPGA environment:

```
bazel build //hello:hello_signed
```

Once built, it can be loaded into an FPGA that has already been pre-loaded with
a bitstream and ROM\_EXT:

```
opentitantool --interface=hyper310 \
    --exec="rescue firmware bazel-bin/hello/hello_bin.prod.signed.bin" \
    console
```
