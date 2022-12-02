use j4rs::{errors::J4RsError, prelude::*, InvocationArg};
use j4rs_derive::*;

type J4RsResult<T> = Result<T, J4RsError>;

#[call_from_java("j4rs.Main.hello")]
fn hello(input: Instance) -> J4RsResult<Instance> {
    // Attach current thread to an active JVM
    let jvm = Jvm::attach_thread().unwrap();
    // Get a Rust String out of a Java String.
    let input: String = jvm.to_rust(input).unwrap();

    // Our reply
    let output = format!("Hello {}", input);

    // Get a Java string out of a Rust String
    let ia = InvocationArg::try_from(output).unwrap();
    Instance::try_from(ia)
}

#[call_from_java("j4rs.Main.dotproduct")]
fn dot_product_native(vector_a: Instance, vector_b: Instance) -> J4RsResult<Instance> {
    // Attach current thread to an active JVM
    let jvm = Jvm::attach_thread().unwrap();
    // Get the vectors from the instances
    // Note, that those are owned vecs, changing them has no effect on the original data.
    let vector_a: Vec<i64> = jvm.to_rust(vector_a).unwrap();
    let vector_b: Vec<i64> = jvm.to_rust(vector_b).unwrap();

    // Compute the dot product
    let dot_product = dot_product(&vector_a, &vector_b);

    // Return the result
    let ia = InvocationArg::try_from(dot_product).unwrap();
    Instance::try_from(ia)
}

#[call_from_java("j4rs.Main.dotproductcallback")]
fn dot_product_callback(vector_a: Instance, vector_b: Instance, callback: Instance) {
    // Attach current thread to an active JVM
    let jvm = Jvm::attach_thread().unwrap();
    // Get the vectors from the instances
    // Note, that those are owned vecs, changing them has no effect on the original data.
    let vector_a: Vec<i64> = jvm.to_rust(vector_a).unwrap();
    let vector_b: Vec<i64> = jvm.to_rust(vector_b).unwrap();

    // Compute the dot product
    let dot_product: i64 = dot_product(&vector_a, &vector_b);
    let ia = InvocationArg::try_from(dot_product).unwrap();

    // Invoke method.
    // Note, that the argument will be received as a boxed Long.
    let _ = jvm.invoke(&callback, "call", &vec![ia]).unwrap();
}

pub struct Counter {
    count: i64,
    callback: Instance,
}

impl Counter {
    pub fn new(callback: Instance) -> Self {
        Self { count: 0, callback }
    }

    pub fn inc(&mut self, jvm: &Jvm) {
        self.count += 1;

        jvm.invoke(
            &self.callback,
            "call",
            &[InvocationArg::try_from(self.count).unwrap()],
        )
        .unwrap();
    }
}

#[call_from_java("j4rs.Main.counternew")]
fn counter_new(callback: Instance) -> J4RsResult<Instance> {
    // Instance in j4rs are GlobalRef, i.e. they are not GCed
    // while we have a reference. We can move the callback
    // into the struct and use it later.
    let counter = Counter::new(callback);

    // Throw the counter on the heap and leak it.
    let ptr = Box::into_raw(Box::new(counter)) as jlong;

    // Return the pointer to the counter
    let ia = InvocationArg::try_from(ptr).unwrap();
    Instance::try_from(ia)
}

#[call_from_java("j4rs.Main.counterinc")]
fn counter_inc(ptr: Instance) {
    // Attach current thread to an active JVM
    let jvm = Jvm::attach_thread().unwrap();
    let ptr: jlong = jvm.to_rust(ptr).unwrap();

    let counter = unsafe { &mut *(ptr as *mut Counter) };

    counter.inc(&jvm);
}

#[call_from_java("j4rs.Main.counterdes")]
fn counter_des(ptr: Instance) {
    // Attach current thread to an active JVM
    let jvm = Jvm::attach_thread().unwrap();
    let ptr: jlong = jvm.to_rust(ptr).unwrap();

    // We dereference the pointer to a boxed counter.
    // The consequence is that if the Box goes out of scope,
    // it's associated memory is being dropped.
    //
    // It'll also drop the wrapped callback which drops
    // the reference to the Java object and makes it GCable.
    let _drop_me = unsafe { Box::from_raw(ptr as *mut Counter) };
}

fn dot_product(a: &[i64], b: &[i64]) -> i64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}
