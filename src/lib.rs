use std::ops::Add;

pub struct ExclusiveRange<T, S> {
    start: T,
    stop: T,
    step: S,
}

pub struct InclusiveRange<T, S> {
    start: T,
    stop: T,
    step: S,
}

pub struct UnboundedRange<T, S> {
    start: T,
    step: S,
}

pub trait StepBy<S> {
    fn step_by(self, step: S) -> Self;
}

impl<T, S> Iterator for ExclusiveRange<T, S>
    where T: PartialOrd + Add<S, Output = T> + Copy,
          S: Copy
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.start < self.stop {
            let tmp = self.start;
            self.start = self.start + self.step;
            Some(tmp)
        } else {
            None
        }
    }
}

impl<T, S> Iterator for InclusiveRange<T, S>
    where T: PartialOrd + Add<S, Output = T> + Copy,
          S: Copy
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.start <= self.stop {
            let tmp = self.start;
            self.start = self.start + self.step;
            Some(tmp)
        } else {
            None
        }
    }
}

impl<T, S> Iterator for UnboundedRange<T, S>
    where T: PartialOrd + Add<S, Output = T> + Copy,
          S: Copy
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let tmp = self.start;
        self.start = self.start + self.step;
        Some(tmp)
    }
}

impl<T, S> StepBy<S> for ExclusiveRange<T, S>
    where T: Add<S, Output = T>
{
    fn step_by(self, step: S) -> Self {
        ExclusiveRange { step: step, ..self }
    }
}

impl<T, S> StepBy<S> for InclusiveRange<T, S>
    where T: Add<S, Output = T>
{
    fn step_by(self, step: S) -> Self {
        InclusiveRange { step: step, ..self }
    }
}

impl<T, S> StepBy<S> for UnboundedRange<T, S>
    where T: Add<S, Output = T>
{
    fn step_by(self, step: S) -> Self {
        UnboundedRange { step: step, ..self }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EPSILON: f32  = 1e-7;

    #[test]
    fn test_excl_int_range() {
        let mut iter: ExclusiveRange<i32, i32> = ExclusiveRange {
            start: 0,
            stop: 3,
            step: 1,
        };
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_excl_float_range() {
        let mut iter: ExclusiveRange<f32, f32> = ExclusiveRange {
            start: 0.0,
            stop: 1.0,
            step: 0.3,
        };
        assert!((iter.next().unwrap().abs() - 0.0) < EPSILON);
        assert!((iter.next().unwrap().abs() - 0.3) < EPSILON);
        assert!((iter.next().unwrap().abs() - 0.6) < EPSILON);
        assert!((iter.next().unwrap().abs() - 0.9) < EPSILON);
        assert_eq!(iter.next(),  None);
    }

    #[test]
    fn test_excl_float_range_on_boundary() {
        let mut iter: ExclusiveRange<f32, f32> = ExclusiveRange {
            start: 0.0,
            stop: 0.9,
            step: 0.3,
        };
        assert!((iter.next().unwrap().abs() - 0.0) < EPSILON);
        assert!((iter.next().unwrap().abs() - 0.3) < EPSILON);
        assert!((iter.next().unwrap().abs() - 0.6) < EPSILON);
        assert_eq!(iter.next(),  None);
    }

    #[test]
    fn test_incl_int_range() {
        let mut iter: InclusiveRange<i32, i32> = InclusiveRange {
            start: 0,
            stop: 3,
            step: 1,
        };
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_incl_float_range() {
        let mut iter: InclusiveRange<f32, f32> = InclusiveRange {
            start: 0.0,
            stop: 1.0,
            step: 0.3,
        };
        assert!((iter.next().unwrap().abs() - 0.0) < EPSILON);
        assert!((iter.next().unwrap().abs() - 0.3) < EPSILON);
        assert!((iter.next().unwrap().abs() - 0.6) < EPSILON);
        assert!((iter.next().unwrap().abs() - 0.9) < EPSILON);
        assert_eq!(iter.next(),  None);
    }

    #[test]
    fn test_incl_float_range_on_boundary() {
        let mut iter: InclusiveRange<f32, f32> = InclusiveRange {
            start: 0.0,
            stop: 0.9,
            step: 0.3,
        };
        assert!((iter.next().unwrap().abs() - 0.0) < EPSILON);
        assert!((iter.next().unwrap().abs() - 0.3) < EPSILON);
        assert!((iter.next().unwrap().abs() - 0.6) < EPSILON);
        // assert!((iter.next().unwrap().abs() - 0.9) < EPSILON);
        assert_eq!(iter.next(),  None);
    }

    #[test]
    fn test_unbound_int_range() {
        let mut iter: UnboundedRange<i32, i32> = UnboundedRange {
            start: 0,
            step: 1,
        };
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        for _ in 1..1000 {
            iter.next();
        }
        assert_eq!(iter.next(), Some(1004));
    }

    #[test]
    fn test_unbound_float_range() {
        let mut iter: UnboundedRange<f32, f32> = UnboundedRange {
            start: 0.0,
            step: 0.3,
        };
        assert!((iter.next().unwrap() - 0.0).abs() < EPSILON);
        assert!((iter.next().unwrap() - 0.3).abs() < EPSILON);
        assert!((iter.next().unwrap() - 0.6).abs() < EPSILON);
        assert!((iter.next().unwrap() - 0.9).abs() < EPSILON);
        assert!((iter.next().unwrap() - 1.2).abs() < EPSILON);
        for _ in 1..1000 {
            iter.next();
        }
        assert!((iter.next().unwrap() - 301.2).abs() < EPSILON);
    }

    #[test]
    fn test_steps_excl_int_range() {
        let mut iter: ExclusiveRange<i32, i32> = ExclusiveRange {
                start: 0,
                stop: 5,
                step: 1,
            }
            .step_by(2);
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);
    }
}
