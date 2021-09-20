use crate::genetic::specimen::Specimen;
use crate::utils::cli::Opts;
use crate::utils::error::Error;
use crate::utils::io;
use crate::utils::reference::Reference;
use crate::utils::time::duration_to_human;
use chrono;
use rand::{self, Rng};
use rayon::prelude::*;

pub struct Population {
    pub generation: u32,
    pub reference: Reference,
    pub specimens: Vec<Specimen>,
    total_score: u32,
    opts: Opts,
    termination_counter: i32,
    start_time: chrono::DateTime<chrono::Utc>,
    report: io::Report,
}

impl Population {
    pub fn new(opts: Opts) -> Result<Population, Error> {
        let reference = Reference::from_file(&opts.input_path)?;

        let generation = 0;

        let mut specimens: Vec<Specimen> = (0..opts.specimens)
            .into_par_iter()
            .map(|_| Specimen::new(&reference))
            .collect();
        specimens.sort_by(|a, b| b.score.cmp(&a.score));

        let total_score = specimens
            .par_iter()
            .map(|specimen| specimen.score)
            .reduce(|| 0, |a, b| a + b);

        Ok(Population {
            generation,
            reference,
            specimens,
            total_score,
            opts,
            termination_counter: 0,
            start_time: chrono::Utc::now(),
            report: io::Report::new(),
        })
    }

    fn next_generation(&mut self) {
        let last_similitude = self.similitude();

        let children = self.create_children();

        let total_score = children
            .par_iter()
            .map(|specimen| specimen.score)
            .reduce(|| 0, |a, b| a + b);

        self.generation += 1;
        self.total_score = total_score;
        self.specimens = children;

        let new_similitude = self.similitude();

        if last_similitude == new_similitude {
            self.termination_counter += 1;
        } else {
            self.termination_counter = 0;
        }

        self.make_report();
    }

    fn similitude(&self) -> f32 {
        let best_specimen = self.specimens.get(0).unwrap();
        best_specimen.similitude
    }

    fn make_report(&mut self) {
        let generation_text = format!("{}/{}", self.generation, self.opts.until);
        let duration = chrono::Utc::now() - self.start_time;
        let time_text = format!("{}", duration_to_human(&duration));
        let simitude_text = format!("{}%", self.similitude());

        self.report
            .insert(String::from("Generation"), generation_text);
        self.report
            .insert(String::from("Process duration"), time_text);
        self.report
            .insert(String::from("Best similitude"), simitude_text);
        self.report.render();
    }

    fn create_children(&self) -> Vec<Specimen> {
        let mut children = Vec::new();
        let number_elite = self.specimens.len() / 5;

        for i in 0..number_elite {
            let parent = self.specimens.get(i).unwrap();
            children.push(parent.clone());
        }

        for _ in number_elite..self.specimens.len() {
            let father = self.spin_wheel();
            let mother = self.spin_wheel();
            children.push(father.make_love(mother, &self.reference))
        }

        children.sort_by(|a, b| b.score.cmp(&a.score));

        children
    }

    fn spin_wheel(&self) -> &Specimen {
        let random_number = rand::thread_rng().gen_range(0, self.total_score);
        let mut current_score = 0;
        for specimen in self.specimens.iter() {
            current_score += specimen.score;
            if random_number <= current_score {
                return specimen;
            }
        }

        panic!("Spin should always select a specimen !");
    }

    pub fn process(&mut self) {
        for _ in 0..self.opts.until {
            self.next_generation();

            if self.termination_counter >= self.opts.convergence {
                break;
            }
        }

        let best_specimen = self.specimens.get(0).unwrap();

        best_specimen
            .render(&self.reference)
            .image
            .save(&self.opts.output_path)
            .expect("Can't save the image result");
    }
}
