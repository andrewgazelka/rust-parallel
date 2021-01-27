/*
https://en.wikipedia.org/wiki/K-means_clustering
Goal reduce within-cluster sum of squares = varaince
 */

use rand::prelude::ThreadRng;
use rand::seq::IteratorRandom;

use crate::points::{Closest, Dist, Mean, Point};

trait KMeans {
    fn group(&self, points: &[Point], group_count: usize) -> Vec<Vec<Point>>;
}

/*
https://en.wikipedia.org/wiki/K-means_clustering#Standard_algorithm_(na%C3%AFve_k-means)
 */
struct NaiveKMeans {
    rng: ThreadRng,
    iterations: usize,
}

impl NaiveKMeans {
    fn new(rdm: ThreadRng, iterations: usize) -> NaiveKMeans {
        NaiveKMeans {
            rng: rdm,
            iterations,
        }
    }
}

impl KMeans for NaiveKMeans {
    fn group(&self, points: &[Point], group_count: usize) -> Vec<Vec<Point>> {
        let mut rng = self.rng.clone();

        let mut groups = vec![vec![]; group_count];

        let mut centroids = points
            .iter()
            .cloned()
            .choose_multiple(&mut rng, group_count);

        for _ in 0..self.iterations {
            groups.iter().for_each(|group| group.clear());
            for point in points {
                // closest centroid
                let closest_idx = centroids.closest_idx(point);

                // add to group
                groups[closest_idx].push(*point);
            }
            centroids = groups.iter().map(|group| group.mean()).collect();
        }
        groups
    }
}
