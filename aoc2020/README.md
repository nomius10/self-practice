[Link to challange website](https://adventofcode.com/2020)

```bash
rustc -C opt-level=z -C prefer-dynamic -C debuginfo=2 main.rs

objdump -d main -M intel -C -S > asm.S
```
