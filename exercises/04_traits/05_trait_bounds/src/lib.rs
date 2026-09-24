// TODO: Add the necessary trait bounds to `min` so that it compiles successfully.
//   Refer to the documentation of the `std::cmp` module for more information on the traits you might need.
//
// Note: there are different trait bounds that'll make the compiler happy, but they come with
// different _semantics_. We'll cover those differences later in the course when we talk about ordered
// collections (e.g. BTreeMap).

/// Return the minimum of two values.
pub fn min<T>(left: T, right: T) -> T 
where 
    T: Ord,
{
    if left <= right {
        left
    } else {
        right
    }
}

/*
trait IsEven {
    pub fn is_even<T: Rem + PartialEq>(item: T)-> bool {
        T % 2 == 0
    }
    or
    /* 
    pub fn is_even<T>(item: T) -> bool 
    where
        T: Rem + PartialEq,
    {
        T % 2 == 0
    };
    */
}

impl IsEven for i32{}
impl IsEven for u32 {}

another alternative is making the trait implementation generic, instead of a generic method above 
so we have 
 trait IsEven {
    fn is _even(&self)-> bool;
 }
 then we say
 impl<T> IsEven for T
 where 
    T: Copy + PartialEq + Rem<i32, Output = T> + num_traits::Zero
{
    fn is_even(&self) -> bool {
        *self % 2 == T::zero()
    }
}
so any type that qualifies this bound gets the IsEven trait implemented and can call the is_even fn()

// in reality we need 
Copy + PartialEq + Rem<i32, Output = T> + num_traits::Zero
 */