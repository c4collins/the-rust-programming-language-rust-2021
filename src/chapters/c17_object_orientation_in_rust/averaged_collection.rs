// An Averaged Collection keeps track of the current average of the contents of it's list.
pub struct AveragedCollection {
    list: Vec<u32>,
    average: f64,
}

impl AveragedCollection {
    // Create an Averaged Collection
    pub fn new() -> Self {
        Self {
            list: vec![],
            average: 0.0,
        }
    }

    // Add an item to the AveragedCollection -> note that the average is recalculated immediately
    pub fn add(&mut self, value: u32) {
        self.list.push(value);
        self.update_average();
    }

    // Remove an item from the AveragedCollection (just using pop()) -> again, the average is updated immedaitely (if something can be removed)
    pub fn delete(&mut self) -> Option<u32> {
        let result = self.list.pop();
        match result {
            Some(num) => {
                self.update_average();
                Some(num)
            }
            None => None,
        }
    }

    // Provide a public getter for the private average field
    pub fn average(&self) -> f64 {
        self.average
    }

    // Provides a private method to update the private average field
    fn update_average(&mut self) {
        let total: u32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
