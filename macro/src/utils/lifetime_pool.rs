use std::collections::HashSet;
use syn::{Ident, Lifetime, LifetimeDef};

pub struct LifetimePool<'a> {
    unused: Vec<&'static str>,
    pub used: Vec<String>,
    occupied: &'a HashSet<String>,
}

impl<'a> LifetimePool<'a> {
    pub fn new(occupied: &'a HashSet<String>) -> LifetimePool {
        let unused = vec!["'_a", "'_b", "'_c", "'_d", "'_e", "'_f", "'_g", "'_h",
                          "'_i", "'_j", "'_k", "'_l", "'_m", "'_n", "'_o", "'_p",
                          "'_q", "'_r", "'_s", "'_t", "'_u", "'_v", "'_w", "'_x",
                          "'_y", "'_z"];
        LifetimePool {
            unused: unused,
            used: vec![],
            occupied: occupied,
        }
    }

    pub fn pop(&mut self) -> String {
        let mut lifetime = None;
        while lifetime.is_none() {
            let candidate = self.unused
                .pop()
                .expect("lifetime pool depleted! you may use too much lifetimes \
                         in your time")
                .to_string();
            if !self.occupied.contains(&candidate) {
                self.used.push(candidate.clone());
                lifetime = Some(candidate);
            }
        }
        lifetime.unwrap()

    }

    pub fn pop_lifetime(&mut self) -> Lifetime {
        Lifetime::new(Ident::new(self.pop()))
    }

    pub fn used_lifetime_def(&self) -> Vec<LifetimeDef> {
        self.used.iter().cloned().map(LifetimeDef::new).collect()
    }
}
