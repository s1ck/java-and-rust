package j4rs;

import org.astonbitecode.j4rs.api.Instance;
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils; 

public class Main {

    // Declares a method that will be provided by a native library.
    // Note, that we need to return the j4rs instance type.
    private static native Instance<String> hello(Instance<String> input);

    // Passing down to primitive arrays and returning a primitive.
    // Not that we have to use lower case method names.
    private static native Instance<Long> dotproduct(Instance<long[]> vectorA, Instance<long[]> vectorB);

    // Passing down primitive arrays and a callback method.
    private static native void dotproductcallback(Instance<long[]> vectorA, Instance<long[]> vectorB, Instance<Callback> callback);
    
    // Instatiate a new instance of "Counter" in native code.
    // The method returns the address of the newly created object.
    private static native Instance<Long> counternew(Instance<Callback> callback);
    // Calls a method on the Counter object in native code
    private static native void counterinc(Instance<Long> counterPtr);
    // Destroys the counter object.
    private static native void counterdes(Instance<Long> counterPtr);

    static {
        // Load the shared object (.so on Linux, .dll on Windows)
        System.loadLibrary("mylib");
    }
    
    public static class DotProductCallback implements Callback {
        public void call(Long result) {
            System.out.println("dot product = " + result);
        }
    }
    
    public static class CounterCallback implements Callback {
        public void call(Long count) {
            System.out.println("count = " + count);
        }
    }

    public static void main(String[] args) {
        // Example 1: Hello World
        Instance<String> greetingInstance = Main.hello(Java2RustUtils.createInstance("Alice"));
        // We need to get the actual instance out of the j4rs Instance
        String greeting = Java2RustUtils.getObjectCasted(greetingInstance);
        System.out.println(greeting);

        // Example 2: Pass two primitive arrays and get a result
        long[] vectorA = {1, 3, 3, 7};
        long[] vectorB = {1, 9, 8, 4};
        var vectorAInstance = Java2RustUtils.createInstance(vectorA);
        var vectorBInstance = Java2RustUtils.createInstance(vectorB);
        Instance<Long> dotProductInstance = Main.dotproduct(vectorAInstance, vectorBInstance);
        Long dotProduct = Java2RustUtils.getObjectCasted(dotProductInstance);
        System.out.println("dot product = " + dotProduct);

        // Example 3: Pass two primitive arrays and a result consumer
        var dotProductCallback = new DotProductCallback();
        Main.dotproductcallback(
            Java2RustUtils.createInstance(vectorA),
            Java2RustUtils.createInstance(vectorB),
            Java2RustUtils.createInstance(dotProductCallback)
        );

        // Example 4: Instantiate an object in native code and interact with it
        Instance<Callback> counterCallback = Java2RustUtils.createInstance(new CounterCallback());
        Instance<Long> counterPtr = Main.counternew(counterCallback);
        // System.out.println("counter ptr = " + counterPtr);
        // Instance<Long> counterPtrInstance = Java2RustUtils.createInstance(counterPtr);
        Main.counterinc(counterPtr);
        Main.counterinc(counterPtr);
        Main.counterdes(counterPtr);
    }
}
