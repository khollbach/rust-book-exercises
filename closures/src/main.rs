use std::rc::Rc;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        *num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.get(intensity));
        println!("Next, do {} situps!", expensive_result.get(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.get(intensity)
            );
        }
    }
}

use std::collections::HashMap;
use std::hash::Hash;

/// todo: It feels like I shouldn't need the Rc here.
pub struct Cacher<F, A, B> {
    f: F,
    cache: HashMap<Rc<A>, B>,
}

impl<F, A, B> Cacher<F, A, B>
where
    F: FnMut(&A) -> B,
    A: Hash + Eq,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            cache: HashMap::new(),
        }
    }

    /// Return f(a). Caches computations so that f is called at most once per value of a.
    ///
    /// todo: It feels like we're doing more lookups than we have to.
    pub fn get(&mut self, a: A) -> &B {
        if self.cache.contains_key(&a) {
            return &self.cache[&a];
        }

        let b = (self.f)(&a);

        // Insert (a, b) into the map.
        let a = Rc::new(a);
        self.cache.insert(Rc::clone(&a), b);

        &self.cache[&a]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a: &u32| -> u32 { *a });

        let _v1 = c.get(1);
        let v2 = c.get(2);

        assert_eq!(*v2, 2);
    }
}
