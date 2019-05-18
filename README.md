# MIPS Simulator.

[![Build Status](https://travis-ci.com/onehr/mips_sim.svg?branch=master)](https://travis-ci.com/onehr/mips_sim)
[![](https://tokei.rs/b1/github/onehr/mips_sim)](https://github.com/onehr/mips_sim)
![GitHub commit activity](https://img.shields.io/github/commit-activity/y/onehr/mips_sim.svg?style=flat)
![GitHub](https://img.shields.io/github/license/onehr/mips_sim.svg)

Note: Now I only decide to support R3000 version.

## Current Process
**Now will only have a top level loop to read assembly instructions and execute them in the simulator**

## TODO:
* CPU and memory objects.
* Should support all `spim` options.
```
	-bare			Bare machine (no pseudo-ops, delayed branches and loads)\n\
	-asm			Extended machine (pseudo-ops, no delayed branches and loads) (default)\n\
	-delayed_branches	Execute delayed branches\n\
	-delayed_loads		Execute delayed loads\n\
	-exception		Load exception handler (default)\n\
	-noexception		Do not load exception handler\n\
	-exception_file <file>	Specify exception handler in place of default\n\
	-quiet			Do not print warnings\n\
	-noquiet		Print warnings (default)\n\
	-mapped_io		Enable memory-mapped IO\n\
	-nomapped_io		Do not enable memory-mapped IO (default)\n\
	-file <file> <args>	Assembly code file and arguments to program\n\
	-assemble		Write assembled code to standard output\n\
	-dump			Write user data and text segments into files\n\
	-full_dump		Write user and kernel data and text into files.\n");
```

## References
* [MIPS](https://www.wikiwand.com/en/MIPS_architecture)
* [R3000](https://www.wikiwand.com/en/R3000)
* [spim](https://www.wikiwand.com/en/SPIM)
