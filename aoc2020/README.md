[Link to challange website](https://adventofcode.com/2020)

```bash
rustc -C opt-level=z -C prefer-dynamic -C debuginfo=2 main.rs

objdump -d main -M intel -C -S > asm.S
```

Repos with solutions of others from a private leaderboard:
- https://github.com/cernat-catalin/advent_of_code_2020
