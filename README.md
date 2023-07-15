# rust_particle_system
CLI tool for the simulation of Interacting Particle Systems. For a theoretical background, see e.g., Stochastic Modeling by Lanchier (2017), chapter 14 for a general introduction, chapter 15 for the contact process (here called the SI process), chapter 16 for the voter process. The simulation algorithm is a generalization of §17.4, Programs 12 and 13 in Lanchier. For a more advanced (measure-theoretic) monograph, see Liggett (1985).

## Quick usage:

* A pretty SIR process development (computation time: ~2 seconds). May need to be run several times to prevent early death.
`--ips-sir 1.0 0.8 --graph-grid-nd 200 200 --initial-different-particles 1 20100 --halt-time-passed 200 --record-constant-time 0.1 --image-gif 200 20 --output "sir 200x200.gif"`
* Higher resolution variant (computation time: ~100 seconds).
`--ips-sir 1.0 0.8 --graph-grid-nd 400 400 --initial-different-particles 1 80200 --halt-time-passed 300 --record-constant-time 0.1 --image-gif 400 20 --output "sir 400x400.gif"`

* A pretty 5-voter process (~4 seconds)
`--ips-voter 5 --graph-grid-nd 60 60 --initial-random --halt-time-passed 200 --record-constant-time 0.1 --image-gif 60 20 --output "5 voter process 60x60.gif"`
* The same at 120x120 resolution takes about 35 seconds. 120x120 for 3 voters takes the same time.

## Slow usage:
The particle systems and graphs are explained in some detail here. For an explanation of the other options defining halting and recording conditions, image output, and initial states, refer to the help function in the CLI

### Particle systems
Four types of interacting particle systems have been implemented:
* The Susceptible-Infected process (aka contact process, SI model, SI process) is a model for an invasive process. A particle can be either infected or susceptible. If a particle is susceptible, neighboring infected particles can make it infected, according to some fixed rate increase per neighbor `birth_rate`. Infected particles transition to susceptible at some fixed `death_rate`. Usage: `--ips-si <BIRTH_RATE> <DEATH_RATE>`.
* The Susceptible-Infected-Removed process models an invasive process with removal. After an infected particle dies, it does not go back to being susceptible but instead becomes removed and cannot be reinfected. Usage: `--ips-si <BIRTH_RATE> <DEATH_RATE>`.
* The Voter process is a model for `n` competitive species (aka parties). Neighboring particles of different parties can convince each other to join their parties, and do so at rate `1.0`. Usage: `--ips-voter <NR_PARTIES>`.
* The Two SI process is a mix of the voter process for 2 species and the SI process. The species are identical. Both mechanisms described there are active for this process. Usage: `--ips-two-si <BIRTH_RATE> <DEATH_RATE> <COMPETE_RATE>`.

More particle systems can be implemented quite easily, see the file `solver/ips_rules.rs` for more information.


### Graphs
Three types of graphs have been implemented:
* The Grid nD graph is a toroidal (i.e., cyclic in each direction) n-dimensional grid. Specify the number of particles in each direction. Usage:  `--graph-grid-nd <X_DIMENSION> <Y_DIMENSION> ...`.
* The Erdos-Renyi graph is a non-spatial graph where two nodes i and j are connected with some probability p. Specify the number of points and the average number of neighbors each point node has. Usage: `--graph-erdos-renyi <NR_NODES> <AVG_NEIGHS_PER_NODES>`.
* The Diluted Lattice graph (aka bond percolation) is a diluted 2d toroidal graph, i.e., two adjacent points i and j in the associated full 2d toroidal graph are connected with probability p. Specify this probability as a percentage. Usage `--graph-diluted-lattice <X_DIMENSION> <Y_DIMENSION> <PERCENTAGE_LINKED>`.

More graphs can be implemented quite easily, see the file `solver/graph.rs` for more information.
