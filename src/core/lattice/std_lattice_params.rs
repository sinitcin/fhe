/*
  Header for the standard values for Lattice Params, as determined by homomorphicencryption.org
 */

use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum DistributionType {
    HEStdUniform,
    HEStdError,
    HEStdTernary,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum SecurityLevel {
    HEStd128Classic,
    HEStd192Classic,
    HEStd256Classic,
    HEStd128Quantum,
    HEStd192Quantum,
    HEStd256Quantum,
    HEStdNotSet,
}

impl fmt::Display for SecurityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SecurityLevel::HEStd128Classic => "HEStd_128_classic",
                SecurityLevel::HEStd192Classic => "HEStd_192_classic",
                SecurityLevel::HEStd256Classic => "HEStd_256_classic",
                SecurityLevel::HEStd128Quantum => "HEStd_128_quantum",
                SecurityLevel::HEStd192Quantum => "HEStd_192_quantum",
                SecurityLevel::HEStd256Quantum => "HEStd_256_quantum",
                SecurityLevel::HEStdNotSet => "HEStd_NotSet",
            }
        )
    }
}

struct StdLatticeParm {
    dist_type: DistributionType,
    ring_dim: usize,
    min_sec_lev: SecurityLevel,
    max_log_q: usize,
}

impl StdLatticeParm {
    fn new(dist_type: DistributionType, ring_dim: usize, min_sec_lev: SecurityLevel, max_log_q: usize) -> Self {
        StdLatticeParm {
            dist_type,
            ring_dim,
            min_sec_lev,
            max_log_q,
        }
    }
}

struct LatticeParamsLookup {
    by_ring: HashMap<(DistributionType, SecurityLevel, usize), StdLatticeParm>,
    by_log_q: HashMap<(DistributionType, SecurityLevel, usize), StdLatticeParm>,
    initialized: bool,
}

impl LatticeParamsLookup {
    fn new() -> Self {
        LatticeParamsLookup {
            by_ring: HashMap::new(),
            by_log_q: HashMap::new(),
            initialized: false,
        }
    }

    fn initialize_lookups(standard_lattice_parm_sets: Vec<StdLatticeParm>) {
        let mut lookup = LatticeParamsLookup::new();
        for s in standard_lattice_parm_sets {
            lookup.by_ring.insert((s.dist_type, s.min_sec_lev, s.ring_dim), s.clone());
            lookup.by_log_q.insert((s.dist_type, s.min_sec_lev, s.max_log_q), s);
        }
        lookup.initialized = true;
    }

    fn find_max_q(dist_type: DistributionType, min_sec_lev: SecurityLevel, ring_dim: usize) -> Option<usize> {
        if !Self::initialized {
            Self::initialize_lookups(vec![]);
        }
        Self::by_ring.get(&(dist_type, min_sec_lev, ring_dim)).map(|s| s.max_log_q)
    }

    fn find_ring_dim(dist_type: DistributionType, min_sec_lev: SecurityLevel, cur_log_q: usize) -> Option<usize> {
        if !Self::initialized {
            Self::initialize_lookups(vec![]);
        }
        let mut prev = 0;
        let mut n = 0;
        for (_, s) in Self::by_log_q.iter().filter(|((dt, msl, _), _)| *dt == dist_type && *msl == min_sec_lev) {
            if cur_log_q <= s.max_log_q && cur_log_q > prev {
                return Some(s.ring_dim);
            }
            prev = s.max_log_q;
            n = s.ring_dim;
        }
        Some(2 * n)
    }
}


