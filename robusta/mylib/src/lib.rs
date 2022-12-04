use robusta_jni::{
    bridge,
    convert::{JavaValue, Signature},
    jni::{objects::JObject, JNIEnv},
};

// bridge attribute enables the module to be processed by robusta
#[bridge]
mod jni {
    use robusta_jni::{convert::Signature, jni::JNIEnv};

    // Create a struct for every Java class that has
    // native methods that we want to implement.
    #[derive(Signature)]
    // Specify full qualified Java package name.
    // We keep it empty since there is no package.
    // otherwise `#[package(com.demo.my.package)]`
    #[package()]
    pub struct Main;

    impl Main {
        // Ordinary Rust function, robusta will do the type conversion
        // for public functions with an jni ABI .. neat.
        pub extern "jni" fn hello(input: String) -> String {
            format!("Hello {input}")
        }

        // The auto-converstion to `Vec<i64>` is only available for
        // ArrayList<T> arguments, unfortunately. For primitve arrays,
        // we'd need to use plain jni.
        pub extern "jni" fn dotProduct(vector_a: Vec<i64>, vector_b: Vec<i64>) -> i64 {
            super::dot_product(&vector_a, &vector_b)
        }

        pub extern "jni" fn dotProductArray<'env>(
            env: &'env JNIEnv,
            vector_a: crate::LongArray<'env>,
            vector_b: crate::LongArray<'env>,
        ) -> i64 {
            // Wrap the pointer to the java array into an AutoArray, which automatically
            // releases the pointer once the variable goes out of scope.
            super::dot_product(&vector_a.to_vec(env), &vector_b.to_vec(env))
        }
    }
}

#[repr(C)]
pub struct LongArray<'env>(JObject<'env>);

impl<'env> Signature for LongArray<'env> {
    const SIG_TYPE: &'static str = "[I)J";
}

impl<'env> JavaValue<'env> for LongArray<'env> {
    fn autobox(
        self,
        _env: &robusta_jni::jni::JNIEnv<'env>,
    ) -> robusta_jni::jni::objects::JObject<'env> {
        todo!()
    }

    fn unbox(
        s: robusta_jni::jni::objects::JObject<'env>,
        _env: &robusta_jni::jni::JNIEnv<'env>,
    ) -> Self {
        Self(s)
    }
}

impl<'env> LongArray<'env> {
    fn to_vec(&'env self, env: &'env JNIEnv) -> Vec<i64> {
        let len = env.get_array_length(self.0.into_inner()).unwrap();
        println!("len = {len}");
        let mut vec = vec![0; len as usize];
        let _ = env.get_long_array_region(self.0.into_inner(), 0, &mut vec);
        println!("vec = {vec:?}");
        vec
    }
}

fn dot_product(a: &[i64], b: &[i64]) -> i64 {
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}
