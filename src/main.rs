pub mod game;
pub mod genetic;
pub mod population;

use crate::population::Population;
use crate::genetic::Point;

fn main() {
    const WIDTH: usize = 5;
    const HEIGHT: usize = 5;

    let mut pop = Population::new(
        100,
        WIDTH,
        HEIGHT,
        vec![
            Point {
                row: 1,
                col: 0,
            },
            Point {
                row: 3,
                col: 0,
            },
        ],
        vec![
            Point {
                row: 2,
                col: 4,
            },
        ],
        Point {
            row: 0,
            col: 4,
        },
        0.5,
    );

    let train = vec![
        vec![vec![0.0, 0.0], vec![0.0]],
        vec![vec![0.0, 1.0], vec![1.0]],
        vec![vec![1.0, 1.0], vec![0.0]],
        vec![vec![1.0, 0.0], vec![1.0]],
    ];

    let mut best = -1000.0;
    let mut b = pop.population[0].clone();

    for iter in 0..1000 {
        if iter % 100 == 0 {
            println!("Iteration: {} Best: {:.5}", iter, best);
        }

        let mut rewards = vec![];

        for ga in pop.population.iter_mut() {
            let mut reward = 0.0;

            for data_point in train.iter() {
                let output = ga.forward(data_point[0].as_slice());
                if ga.nn.cells[ga.activator.row][ga.activator.col].spiked {
                    reward += 1.0;
                }
                reward += -((output[0] - data_point[1][0]).abs());
            }

            rewards.push(reward);
        }

        for (i, reward) in rewards.iter().enumerate() {
            pop.add_rew(*reward);
            if *reward > best {
                best = *reward;
                b = pop.population[i].clone();
            }
        }

        pop.learn();
    }

    for data_point in train.iter() {
        let output = b.forward(data_point[0].as_slice());
        println!("Input: {:?} Output: {:.5}", data_point[0], output[0]);
    }
}
