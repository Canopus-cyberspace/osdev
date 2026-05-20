# Judge-Driven Development Rules

## Rule 1: read the judge first

Before implementing a suite, inspect:

```text
/home/lenovo/oscomp-official-env/autotest-for-oskernel/kernel/judge/<judge>.py
```

## Rule 2: inspect official serial parsing

Before emitting group output, inspect:

```text
/home/lenovo/oscomp-official-env/autotest-for-oskernel/kernel/parse_output_2023.py
```

## Rule 3: verify official test content

Before claiming a subtest, verify official test script or binary content.

## Rule 4: no fake group output

Official group output must be tied to real official content or real execution.

## Rule 5: score regression protection

After every batch, run the official harness and compare:

- total score;
- target suite score;
- previous nonzero suite score.
