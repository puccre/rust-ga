mod ga;
use ga::{Chromosome, Population};
use rand::Rng;

fn main() {
    let mut pop: Population<Action> = Population::new();
    for _ in 0..100 {
        let mut chromosome: Chromosome<Action> = Chromosome {
            genes: Vec::new(),
            score: 0.0,
        };
        for _ in 0..5 {
            chromosome.genes.push(Action {
                x: rand::thread_rng().gen_range(0, 6) as f64,
            });
        }
        pop.chromosomes.push(chromosome);
    }

    pop.fittness(&fitness);
    println!("initial score: {}", pop.chromosomes[0].score);

    for gen in 0..10 {
        pop.next_gen(&cross_over);
        pop.fittness(&fitness);
        println!("score in gen {}: {}", gen, pop.chromosomes[0].score);
    }

    println!("final score: {}", pop.chromosomes[0].score);
    println!("final score: {:?}", pop.chromosomes[0].genes);
}

#[derive(Clone, Debug)]
struct Action {
    x: f64,
}

impl ga::Gene for Action {
    fn mutate(&mut self) {
        self.x = rand::thread_rng().gen_range(0, 6) as f64;
    }
}

fn fitness(chromosome: &ga::Chromosome<Action>) -> f64 {
    let mut sum = 1.0;
    for i in 0..chromosome.genes.len() {
        sum += 5.0 - chromosome.genes[i].x;
    }
    1.0 / sum
}

fn cross_over(
    parent_1: &ga::Chromosome<Action>,
    parent_2: &ga::Chromosome<Action>,
) -> (ga::Chromosome<Action>, ga::Chromosome<Action>) {
    let mut genes_1 = Vec::with_capacity(parent_1.genes.len());
    let mut genes_2 = Vec::with_capacity(parent_1.genes.len());
    for i in 0..parent_1.genes.len() {
        genes_1.push(Action {
            x: (0.8 * parent_1.genes[i].x + (1.0 - 0.8) * parent_2.genes[i].x).round(),
        });
        genes_2.push(Action {
            x: (0.8 * parent_2.genes[i].x + (1.0 - 0.8) * parent_1.genes[i].x).round(),
        });
    }
    (
        Chromosome {
            genes: genes_1,
            score: 0.0,
        },
        Chromosome {
            genes: genes_2,
            score: 0.0,
        },
    )
}
