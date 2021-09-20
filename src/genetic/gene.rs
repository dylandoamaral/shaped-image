use crate::utils::reference::Reference;
use image::Rgb;
use num::clamp;
use rand::{
    self,
    distributions::{uniform::SampleUniform, Standard},
    prelude::Distribution,
    Rng,
};
use std::ops::Add;

#[derive(Copy, Clone)]
pub struct Gene {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
    pub color: Rgb<u8>,
    pub width: u32,
    pub height: u32,
}

fn maybe_mutate<T>(value: T, mutation_chance: f32, low: T, high: T, min: T, max: T) -> T
where
    Standard: Distribution<T>,
    T: PartialOrd + SampleUniform + Add<Output = T>,
{
    if rand::thread_rng().gen::<f32>() < mutation_chance {
        let offset = rand::thread_rng().gen_range(low, high);
        clamp(value + offset, min, max)
    } else {
        value
    }
}

impl Gene {
    pub fn new(reference: &Reference) -> Gene {
        let (width, height) = (reference.width(), reference.height());
        // TODO: Modify this number
        let radius = rand::thread_rng().gen_range(5, 10);
        let x = rand::thread_rng().gen_range(radius, width - radius) as i32;
        let y = rand::thread_rng().gen_range(radius, height - radius) as i32;
        let color = reference.random_color();

        Gene {
            x,
            y,
            radius: radius as i32,
            color,
            width,
            height,
        }
    }

    pub fn from_gene_with_mutation(gene: &Gene, reference: &Reference) -> Gene {
        // TODO: Modify this number
        let mutation_chance = 0.01;
        let radius = maybe_mutate(gene.radius, mutation_chance, -2, 2, 5, 10);

        let x = maybe_mutate(
            gene.x,
            mutation_chance,
            -20,
            20,
            radius,
            gene.width as i32 - radius,
        );
        let y = maybe_mutate(
            gene.y,
            mutation_chance,
            -20,
            20,
            radius,
            gene.height as i32 - radius,
        );

        let color = if rand::thread_rng().gen::<f32>() < mutation_chance {
            reference.random_color()
        } else {
            gene.color
        };

        Gene {
            x,
            y,
            radius,
            color,
            width: gene.width,
            height: gene.height,
        }
    }

    pub fn crossover(&self, other: &Gene, reference: &Reference) -> Gene {
        let random_bool = rand::random::<bool>();
        let parent_gen = if random_bool { self } else { other };
        Gene::from_gene_with_mutation(parent_gen, reference)
    }
}
