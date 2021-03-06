//! This module contains the Crossover trait and the provided crossover functions.

use genotype::Genotype;
use rand::distributions::Uniform;
use rand::prelude::*;
use std::cmp::{min, PartialEq};
use CrossoverFunctions::*;

/// This trait defines the cross function.
pub trait Crossover<T: PartialEq, G: Genotype<T>>: Send + Sync {
    /// Generates two children combining the two selected individuals.
    fn cross(&self, ind1: &G, ind2: &G) -> (G, G);
}

/// Provided crossover functions.
#[derive(Debug)]
pub enum CrossoverFunctions {
    /// Single point Crossover.
    SingleCrossPoint,
    /// Multi point Crossover.
    MultiCrossPoint,
    /// Uniform Crossover.
    UniformCross,
}

impl<T: PartialEq, G: Genotype<T>> Crossover<T, G> for CrossoverFunctions {
    fn cross(&self, ind1: &G, ind2: &G) -> (G, G) {
        match self {
            SingleCrossPoint => {
                let ind_size = min(ind1.iter().len(), ind2.iter().len());
                let cross_point = SmallRng::from_entropy().sample(Uniform::from(1..ind_size));

                let mut child1 = ind1.clone();
                child1.from_iter(
                    ind1.clone()
                        .into_iter()
                        .zip(ind2.clone().into_iter())
                        .enumerate()
                        .map(|(i, (gen1, gen2))| if i < cross_point { gen1 } else { gen2 }),
                );
                let mut child2 = ind2.clone();
                child2.from_iter(
                    ind1.clone()
                        .into_iter()
                        .zip(ind2.clone().into_iter())
                        .enumerate()
                        .map(|(i, (gen1, gen2))| if i < cross_point { gen2 } else { gen1 }),
                );

                (child1, child2)
            }
            MultiCrossPoint => {
                let ind_size = min(ind1.iter().len(), ind2.iter().len());
                let mut cross_points = Vec::new();
                let mut point_maximum = ind_size / 2;
                if point_maximum <= 2 {
                    point_maximum = 3;
                }
                let mut i = SmallRng::from_entropy().sample(Uniform::from(1..point_maximum));
                while i < ind_size {
                    cross_points.push(i);
                    i += SmallRng::from_entropy().sample(Uniform::from(1..point_maximum));
                }
                cross_points.push(ind_size);

                let mut child1 = ind1.clone();
                child1.from_iter(
                    ind1.clone()
                        .into_iter()
                        .zip(ind2.clone().into_iter())
                        .enumerate()
                        .map(|(i, (gen1, gen2))| {
                            let mut even = false;
                            for cross_point in &cross_points {
                                if i < *cross_point {
                                    if even {
                                        return gen2;
                                    } else {
                                        return gen1;
                                    }
                                } else {
                                    even = !even;
                                }
                            }
                            gen1
                        }),
                );
                let mut child2 = ind2.clone();
                child2.from_iter(
                    ind1.clone()
                        .into_iter()
                        .zip(ind2.clone().into_iter())
                        .enumerate()
                        .map(|(i, (gen1, gen2))| {
                            let mut even = false;
                            for cross_point in &cross_points {
                                if i < *cross_point {
                                    if even {
                                        return gen1;
                                    } else {
                                        return gen2;
                                    }
                                } else {
                                    even = !even;
                                }
                            }
                            gen2
                        }),
                );

                (child1, child2)
            }
            UniformCross => {
                let mut child1 = ind1.clone();
                child1.from_iter(
                    ind1.clone()
                        .into_iter()
                        .zip(ind2.clone().into_iter())
                        .enumerate()
                        .map(|(i, (gen1, gen2))| if i % 2 == 0 { gen1 } else { gen2 }),
                );
                let mut child2 = ind2.clone();
                child2.from_iter(
                    ind1.clone()
                        .into_iter()
                        .zip(ind2.clone().into_iter())
                        .enumerate()
                        .map(|(i, (gen1, gen2))| if i % 2 != 0 { gen1 } else { gen2 }),
                );

                (child1, child2)
            }
        }
    }
}
