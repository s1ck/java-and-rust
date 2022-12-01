use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Main interface to the JVM.
use jni::JNIEnv;
// Objects we need to use as arguments to the native function.
// Carry extra lifetime info to prevent them escaping this
// context and gettung used after being GC'd.
use jni::objects::{AutoArray, GlobalRef, JClass, JObject, JString, ReleaseMode};
// A pointer which we use as return type. This is necessary since
// we cannot return objects with lifetime annotations.
use jni::sys::{jlong, jlongArray, jstring};

// Keeps Rust from "mangling" the name, e.g., by making it unique
// within this crate.
#[no_mangle]
pub extern "system" fn Java_Main_hello(env: JNIEnv, _class: JClass, input: JString) -> jstring {
    // _class refers to the class that owns the static method. It's not used, but we need
    // to match the expected signature of a static native method.

    // Get a Rust String out of a Java String.
    let input: String = env
        .get_string(input)
        .expect("Couldn't get java string")
        .into();

    // Our reply.
    let output = format!("Hello {}", input);

    // Get a Java String out of a Rust String.
    let output: JString = env
        .new_string(output)
        .expect("Couldn't create a Java string");

    // Get the raw (pointer) type, which is jstring.
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_Main_dotProduct(
    env: JNIEnv,
    _class: JClass,
    vector_a: jlongArray,
    vector_b: jlongArray,
) -> jlong {
    // Wrap the pointer to the java array into an AutoArray, which automatically
    // releases the pointer once the variable goes out of scope.
    let vector_a: AutoArray<i64> = env
        .get_long_array_elements(vector_a, ReleaseMode::NoCopyBack)
        .expect("Couldn't create long array for vector A");

    let vector_b: AutoArray<i64> = env
        .get_long_array_elements(vector_b, ReleaseMode::NoCopyBack)
        .expect("Couldn't create long array for vector B");

    println!("vector_a is_copy = {}", vector_a.is_copy());
    println!("vector_b is_copy = {}", vector_b.is_copy());

    let vector_a: &[i64] =
        unsafe { std::slice::from_raw_parts(vector_a.as_ptr(), vector_a.size().unwrap() as usize) };

    let vector_b: &[i64] =
        unsafe { std::slice::from_raw_parts(vector_b.as_ptr(), vector_b.size().unwrap() as usize) };

    dot_product(vector_a, vector_b)
}

#[no_mangle]
pub extern "system" fn Java_Main_dotProductCritical(
    env: JNIEnv,
    _class: JClass,
    vector_a: jlongArray,
    vector_b: jlongArray,
) -> jlong {
    // Wrap the pointer to the java array into an AutoArray, which automatically
    // releases the pointer once the variable goes out of scope.
    let vector_a = env
        .get_primitive_array_critical(vector_a, ReleaseMode::NoCopyBack)
        .expect("Couldn't create primitive auto array for vector A");

    let vector_b = env
        .get_primitive_array_critical(vector_b, ReleaseMode::NoCopyBack)
        .expect("Couldn't create primitive auto array for vector B");

    println!("vector_a is_copy = {}", vector_a.is_copy());
    println!("vector_b is_copy = {}", vector_b.is_copy());

    let vector_a: &[i64] = unsafe {
        std::slice::from_raw_parts(
            vector_a.as_ptr() as *const _,
            vector_a.size().unwrap() as usize,
        )
    };

    let vector_b: &[i64] = unsafe {
        std::slice::from_raw_parts(
            vector_b.as_ptr() as *const _,
            vector_b.size().unwrap() as usize,
        )
    };

    dot_product(vector_a, vector_b)
}

#[no_mangle]
pub extern "system" fn Java_Main_dotProductConsume(
    env: JNIEnv,
    _class: JClass,
    vector_a: jlongArray,
    vector_b: jlongArray,
    callback: JObject,
) {
    // Wrap the pointer to the java array into an AutoArray, which automatically
    // releases the pointer once the variable goes out of scope.
    let vector_a: AutoArray<i64> = env
        .get_long_array_elements(vector_a, ReleaseMode::NoCopyBack)
        .expect("Couldn't create long array for vector A");

    let vector_b: AutoArray<i64> = env
        .get_long_array_elements(vector_b, ReleaseMode::NoCopyBack)
        .expect("Couldn't create long array for vector B");

    let vector_a: &[i64] =
        unsafe { std::slice::from_raw_parts(vector_a.as_ptr(), vector_a.size().unwrap() as usize) };

    let vector_b: &[i64] =
        unsafe { std::slice::from_raw_parts(vector_b.as_ptr(), vector_b.size().unwrap() as usize) };

    let dot_product = dot_product(vector_a, vector_b);

    env.call_method(callback, "call", "(J)V", &[dot_product.into()])
        .unwrap();
}

pub struct Counter {
    count: i64,
    callback: GlobalRef,
}

impl Counter {
    pub fn new(callback: GlobalRef) -> Self {
        Self { count: 0, callback }
    }

    pub fn inc(&mut self, env: JNIEnv) {
        self.count += 1;
        env.call_method(&self.callback, "call", "(J)V", &[self.count.into()])
            .unwrap();
    }
}

#[no_mangle]
pub extern "system" fn Java_Main_counterNew(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
) -> jlong {
    // Turn the callback object into a global reference.
    // This "pins" the object and stops GC from collecting it.
    // The object is unpinned when the global ref is dropped.
    let callback = env.new_global_ref(callback).unwrap();
    let counter = Counter::new(callback);

    // We put the counter object on the heap and immediately
    // leak the object and return the pointer to it.
    // We are now responsible for deallocating it.
    Box::into_raw(Box::new(counter)) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_Main_counterInc(
    env: JNIEnv,
    _class: JClass,
    counter_ptr: jlong,
) {
    // We interpret the given pointer as a pointer to a Counter object.
    // We dereference the pointer and mutably borrow the object.
    let counter = &mut *(counter_ptr as *mut Counter);
    counter.inc(env);
}

#[no_mangle]
pub unsafe extern "system" fn Java_Main_counterDes(
    _env: JNIEnv,
    _class: JClass,
    counter_ptr: jlong,
) {
    // We dereference the pointer to a boxed counter.
    // The consequence is that if the Box goes out of scope,
    // it's associated memory is being dropped.
    let _drop_me = Box::from_raw(counter_ptr as *mut Counter);
}

#[no_mangle]
pub extern "system" fn Java_Main_asyncComputation(env: JNIEnv, _class: JClass, callback: JObject) {
    // JNIEnv is not Send, i.e. we cannot send it across thread boundaries, but JavaVM is.
    let jvm = env.get_java_vm().unwrap();

    // Pin the callback object to avoid it getting GC'ed.
    let callback = env.new_global_ref(callback).unwrap();

    // Using a channel to block the native call until the native thread has started.
    // Otherwise, the method returns and the Java program could finish before the
    // native thread started.
    let (tx, rx) = mpsc::channel();

    let _ = thread::spawn(move || {
        // Signal that the thread has started.
        tx.send(()).unwrap();

        // Actual work starts here.

        // Attach a JNIEnv to the current thread.
        let env = jvm.attach_current_thread().unwrap();

        for i in 0..=10 {
            let progress = (i * 10) as jlong;
            // Send progress to the Java callback
            env.call_method(&callback, "call", "(J)V", &[progress.into()])
                .unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    // Block until the thread started.
    rx.recv().unwrap();
}

fn dot_product(a: &[i64], b: &[i64]) -> i64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}
