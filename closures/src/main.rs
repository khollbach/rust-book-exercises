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

pub struct Cacher<F, A, B> {
    f: F,
    cache: HashMap<A, B>,
}

impl<F, A, B> Cacher<F, A, B>
where
    F: Fn(&A) -> B,
    A: Eq + Hash,
    A: Clone, // todo this shouldn't be necessary..
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            cache: HashMap::new(),
        }
    }

    // todo how would I write this fn so that I don't have to clone a?
    // I should conceptually be able to give ownership of a and b to the
    // map while simultaneously borrowing b back from the map. But it's
    // not obvious whether this is possible, because to borrow b from the
    // map I still need a reference to a for the lookup.
    pub fn get(&mut self, a: A) -> &B {
        if !self.cache.contains_key(&a) {
            let b = (self.f)(&a);
            self.cache.insert(a.clone(), b);
        }

        self.cache.get(&a).unwrap()
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

    // todo: is it possible to use this cacher with
    // something like substr(&a) -> &b ???
    //#[test]
    //fn substr() {
    //fn substr(s: &String) -> &str {
    //&s[..1]
    //}
    //let s = "abc".to_string();

    //let mut c = Cacher::new(substr);
    //let x: &str = c.get(s);
    //assert_eq!(x, "a");
    //}
}
