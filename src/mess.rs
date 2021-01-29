#![feature(const_generics, const_evaluatable_checked, array_map)]
fn split_head<T, const N: usize>(array: [T; N]) -> (T, [T; N - 1])
    where [T; N - 1]:
{
    let array = std::mem::ManuallyDrop::new(array);
    unsafe {
        (
            std::ptr::read(&array[0]),
            array.as_ptr().cast::<T>().offset(1).cast::<[T; N - 1]>().read(),
        )
    }
}