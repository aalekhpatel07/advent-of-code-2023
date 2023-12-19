pub type MinHeap<T, State> = std::collections::BinaryHeap<(Cost<T>, State)>;

#[derive(Debug, Clone, Copy)]
pub struct Cost<T>(T);

impl<T> Cost<T> 
{
    pub fn value(&self) -> &T {
        &self.0
    }
}


impl<T> PartialEq for Cost<T> 
where
    T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Eq for Cost<T> 
where
    T: Eq {}

impl<T> PartialOrd for Cost<T>
where T: PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(std::cmp::Ordering::Equal) => Some(std::cmp::Ordering::Equal),
            Some(std::cmp::Ordering::Greater) => Some(std::cmp::Ordering::Less),
            Some(std::cmp::Ordering::Less) => Some(std::cmp::Ordering::Greater),
            None => None
        }
    }
}


impl<T> Ord for Cost<T>
where
    T: Ord {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match self.0.cmp(&other.0) {
                std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
                std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
                std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
            }
        }
    }

