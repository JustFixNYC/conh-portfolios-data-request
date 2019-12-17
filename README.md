This is an attempt to collect data about the landlord portfolios
associated with New York City's
[Certification of No Harassment (CONH) Pilot Building List][conh].

## Methodology

We use data from [Who Owns What (WoW)][wow] to correlate BBLs in the
CONH list with data about the portfolios rooted in each BBL. Specifically,
for each BBL, WoW will:

1. gather the business address (or addresses) of entities registered with the BBL on HPD and finds other properties in the city with a matching address, and
2. gather the "Head Officer", "Individual Owner", and "Corporate Owner" contact names registered with the BBL on HPD and find other properties in the city that have a matching contact name, using "fuzzy matching" to account for misspellings.

This program then attempts to group the properties and their portfolios
into a set of disjoint "virtual portfolios".  This is done by
creating a uni-directional graph structure in which nodes are
BBLs and edges between two BBLs exist if one of their portfolios
contains the other BBL.

For example, if BBL `a` has a WoW portfolio consisting of BBLs `b`, `c`, 
and `d`, while BBL `x` has a WoW portfolio consisting of BBLs `y`, `z`, and 
`d`, then both buildings will be in the same virtual portfolio because they
are linked through the common BBL `d`.

[wow]: https://whoownswhat.justfix.nyc/

## Output

The latest output from running the script can be found at
[`data/output.csv`](./data/output.csv).

## Quick start

You will need [Rust](https://www.rust-lang.org/).

```
cargo run
```

The repository currently contains cached data from WoW's API.
If you want to refresh this data, delete the contents of
`data/wow/` and re-run the program.

[conh]: https://data.cityofnewyork.us/Housing-Development/Certification-of-No-Harassment-CONH-Pilot-Building/bzxi-2tsw
