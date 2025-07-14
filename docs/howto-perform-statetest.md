# How to Perform State Tests

## Prepare

Install the required dependencies:

```bash
make bootstrap
```

## Build

Build the measurement tools:

```bash
make build-measure
```

## Run

Execute the state tests:

```bash
make run-measure
```

## Troubleshooting

If you need to debug issues, you can use the `--instructions` option to limit the number of executed instructions, and combine it with `-l` and `--log` to generate a detailed execution log:

```bash
spike --instructions 100000000 -l --log=./trace.log --isa=rv64imac ./target/riscv64imac-unknown-none-elf/release/statetest-measure
```