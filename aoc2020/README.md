[Link to challange website](https://adventofcode.com/2020)

```bash
rustc -C opt-level=z -C prefer-dynamic -C debuginfo=2 main.rs

objdump -d main -M intel -C -S > asm.S
```

Repos with solutions of other participants from a private leaderboard:
- https://github.com/cernat-catalin/advent_of_code_2020
- https://github.com/raresrosca/advent_of_code_2020
